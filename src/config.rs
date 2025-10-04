use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub lat: f64,
    pub lon: f64,
    pub units: String,
    pub lang: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: "".to_string(),
            lat: 0.0,
            lon: 0.0,
            units: "metric".to_string(),
            lang: "en".to_string(),
        }
    }
}

/// Get the platform-appropriate paths for config and DB
pub fn get_paths() -> (PathBuf, PathBuf) {
    let proj_dirs = ProjectDirs::from("", "", "weather_fetcher")
        .expect("Could not determine project directories");

    // Ensure directories exist
    fs::create_dir_all(proj_dirs.config_dir()).expect("Failed to create config dir");
    fs::create_dir_all(proj_dirs.data_dir()).expect("Failed to create data dir");

    // Define files
    let config_file = proj_dirs.config_dir().join("config.toml");
    let db_file = proj_dirs.data_dir().join("weather.db");

    (config_file, db_file)
}

pub fn load_config() -> Config {
    let (config_path, _) = get_paths();

    // If the file doesn't exist, create a template
    if !config_path.exists() {
        println!(
            "Config file not found. Creating template at {}",
            config_path.display()
        );
        let default_config = Config::default();
        let toml_str =
            toml::to_string_pretty(&default_config).expect("Failed to serialize default config");

        let mut file = fs::File::create(&config_path).expect("Failed to create config file");
        file.write_all(toml_str.as_bytes())
            .expect("Failed to write to config file");

        return default_config;
    }

    let contents = fs::read_to_string(&config_path).expect("Failed to read config file");

    toml::from_str(&contents).expect("Failed to parse config file")
}

/// Get the path to the SQLite DB
pub fn get_db_path() -> PathBuf {
    let (_, db_path) = get_paths();
    db_path
}
