use std::fs;
use std::path::PathBuf;

use crate::types::Result;
use crate::utils::get_ca_certs_folder;

struct Config {
    key_src: PathBuf,
    cert_src: PathBuf,
    key_dest: PathBuf,
    cert_dest: PathBuf,
}

impl Config {
    fn new(cert_src: String, key_src: String) -> Self {
        Self {
            key_src: key_src.into(),
            cert_src: cert_src.into(),
            key_dest: get_ca_certs_folder().join("key.pem"),
            cert_dest: get_ca_certs_folder().join("cert.pem"),
        }
    }

    fn commit(self) -> Result<()> {
        fs::copy(self.key_src, self.key_dest)?;
        fs::copy(self.cert_src, self.cert_dest)?;

        Ok(())
    }
}

pub fn call(cert_path: String, key_path: String) -> Result<()> {
    let config = Config::new(cert_path, key_path);
    config.commit()?;

    Ok(())
}
