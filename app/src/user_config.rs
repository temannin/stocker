use home::home_dir;
use serde::{Deserialize, Serialize};
use slog_scope::info;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub(crate) device_path: String,
    pub(crate) upc_api_key: String,
}

// Define default configuration values
impl Default for Config {
    fn default() -> Self {
        Config {
            device_path: "".to_string(),
            upc_api_key: "".to_string(),
        }
    }
}

pub fn load_config() -> Config {
    // Path to config file, starting with ~
    let path = "~/stocker/config.json".to_string();

    // Expand ~ to the home directory
    let expanded_path = if path.starts_with("~") {
        let home = home_dir().expect("Failed to find home directory");
        PathBuf::from(path.replacen("~", home.to_str().unwrap(), 1))
    } else {
        PathBuf::from(path)
    };

    // Check if the config file exists
    if !expanded_path.exists() {
        // If not, create it with default values
        info!(
            "Config file not found at {:?}. Creating default config.",
            expanded_path
        );

        let default_config = Config::default();
        let json_data = serde_json::to_string_pretty(&default_config)
            .expect("Failed to serialize default config");

        // Create the parent directory if it doesn't exist
        if let Some(parent) = expanded_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create config directory");
        }

        // Write default config to the file
        let mut file = fs::File::create(&expanded_path).expect("Failed to create config file");
        file.write_all(json_data.as_bytes())
            .expect("Failed to write default config to file");

        info!("Created default config file at {:?}", expanded_path);
    } else {
        info!(
            "Config file found at {:?}. Loading configuration.",
            expanded_path
        );
    }

    // Read the config file
    let config_data = fs::read_to_string(&expanded_path)
        .unwrap_or_else(|_| panic!("Failed to read config file at {:?}", expanded_path));

    // Deserialize JSON into Config struct
    info!("Deserializing config file content...");
    let config: Config = serde_json::from_str(&config_data).expect("Failed to parse JSON");

    info!("Config loaded successfully.");
    config
}
