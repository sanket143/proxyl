mod handler;

use hudsucker::certificate_authority::OpensslAuthority;
use hudsucker::openssl::hash::MessageDigest;
use hudsucker::openssl::pkey::PKey;
use hudsucker::openssl::x509::X509;
use hudsucker::Proxy;
use notify::{event, EventKind};
use notify::{RecursiveMode, Watcher};
use std::fs;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use toml::{map::Map, Value};

use crate::types::error::ProxylError;
use crate::types::Result;
use crate::utils::{get_ca_certs_folder, get_config_folder};
use handler::Handler;

fn rule_handler(value: Map<String, Value>) -> Map<String, Value> {
    let mut value = value;
    let empty_table = Value::Table(Map::new());
    let config = value
        .get("config")
        .unwrap_or(&empty_table)
        .as_table()
        .unwrap()
        .clone();
    let rules = value["rules"].as_table_mut().unwrap();

    for (_, rule) in rules.iter_mut() {
        if let Some(value) = rule.get("config") {
            let config_name = value.as_str().unwrap();
            let map = rule.as_table().unwrap().to_owned();
            let mut local_config = config
                .get(config_name)
                .expect("No such config found")
                .as_table()
                .unwrap()
                .to_owned();
            local_config.extend(map);
            *rule = local_config.into();
        }
    }

    rules.to_owned()
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

pub async fn call() -> Result<()> {
    let ca_folder = get_ca_certs_folder();
    let config_folder = get_config_folder();
    let config_file = Arc::new(config_folder.join("config.toml"));
    let password = rpassword::prompt_password("Key passphrase: ").unwrap();

    // clones for different threads
    let contents = fs::read_to_string(config_file.as_ref()).unwrap_or("".to_owned());
    let rules = Arc::new(Mutex::new(rule_handler(
        contents.parse::<toml::Table>().unwrap(),
    )));
    let config_file_watcher = config_file.clone();
    let handler = Handler::new(rules.clone());
    let handler_watcher = rules.clone();

    let mut watcher =
        notify::recommended_watcher(move |res: notify::Result<event::Event>| match res {
            Ok(event) => {
                if let EventKind::Modify(event::ModifyKind::Data(event::DataChange::Content)) =
                    event.kind
                {
                    let contents = fs::read_to_string(config_file_watcher.as_ref());

                    if let Err(err) = contents {
                        // don't process further
                        println!("Can't read the configuration file, {}", err);
                        return;
                    }

                    {
                        let contents = contents.unwrap().parse::<toml::Table>();

                        if let Err(err) = contents {
                            // invalid TOML file
                            println!("Invalid TOML file: {}", err);
                            return;
                        }

                        *handler_watcher.lock().unwrap() = rule_handler(contents.unwrap());
                    }

                    println!("Config updated!");
                }
            }
            Err(e) => {
                println!("watch error: {:?}", e);
            }
        })
        .unwrap();

    // monitor config file for changes.
    watcher
        .watch(&config_file, RecursiveMode::Recursive)
        .unwrap();

    let private_key_bytes = std::fs::read(ca_folder.join("key.pem")).unwrap();
    let ca_cert_bytes = std::fs::read(ca_folder.join("cert.pem")).unwrap();
    let private_key =
        PKey::private_key_from_pem_passphrase(&private_key_bytes, password.as_str().as_bytes())
            .expect("Failed to parse private key");
    println!("Authorized!");
    let ca_cert = X509::from_pem(&ca_cert_bytes).expect("Failed to parse CA certificate");
    let ca = OpensslAuthority::new(private_key, ca_cert, MessageDigest::sha256(), 1_000);

    let proxy = Proxy::builder()
        .with_addr(SocketAddr::from(([127, 0, 0, 1], 8080)))
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(handler)
        .build();

    if let Err(e) = proxy.start(shutdown_signal()).await {
        return Err(ProxylError::new(e));
    }

    Ok(())
}
