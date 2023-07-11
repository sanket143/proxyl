use clap::{Parser, Subcommand};

/// Proxy outgoing requests to other servers selectively
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Port number for the proxy server
    #[arg(short, long, default_value_t = 8080)]
    pub(crate) port: u16,

    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configure options for proxyl
    Config {
        #[arg(short, long, default_value_t = String::from("./cert.pem"))]
        cert_path: String,

        #[arg(short, long, default_value_t = String::from("./key.pem"))]
        key_path: String,
    },

    /// Start proxyl server
    Serve,
}
