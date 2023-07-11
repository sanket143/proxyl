use std::fs;
use std::path::PathBuf;

pub fn get_config_folder() -> PathBuf {
    let config_folder = dirs::home_dir().unwrap().join(".config").join("proxyl");

    if !config_folder.exists() {
        fs::create_dir_all(&config_folder).expect("Failed to create directory for configuration");
    }

    config_folder
}

pub fn get_ca_certs_folder() -> PathBuf {
    let folder = get_config_folder().join("ca");

    if !folder.exists() {
        fs::create_dir_all(&folder).expect("Failed to create directory for certificates");
    }

    folder
}
