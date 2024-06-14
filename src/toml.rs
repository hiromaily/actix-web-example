use serde::Deserialize;
use std::error::Error;
use std::fs;

// toml definition
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[allow(dead_code)]
    pub app_name: String,
    #[allow(dead_code)]
    pub server: Server,
    #[allow(dead_code)]
    pub logger: Logger,
    #[allow(dead_code)]
    pub db: PostgreSQL,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    #[allow(dead_code)]
    pub host: String,
    #[allow(dead_code)]
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    #[allow(dead_code)]
    service: String,
    #[allow(dead_code)]
    level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostgreSQL {
    #[allow(dead_code)]
    pub enabled: bool,
    #[allow(dead_code)]
    pub host: String,
    #[allow(dead_code)]
    pub dbname: String,
    #[allow(dead_code)]
    pub user: String,
    #[allow(dead_code)]
    pub password: String,
}

// print loaded config
#[allow(dead_code)]
pub fn print_loaded_config(file_name: &str) {
    let toml_str =
        fs::read_to_string(file_name).unwrap_or_else(|_| panic!("Failed to read {}", file_name));
    let config: Config =
        toml::from_str(&toml_str).unwrap_or_else(|_| panic!("Failed to deserialize {}", file_name));
    println!("{:#?}", config);
}

// return loaded config
// example to call
// ```
// match toml::load_config("./config/settings.toml") {
//   Ok(config) => println!("{:#?}", config),
//   Err(e) => {
//       eprintln!("Error loading config: {}", e);
//       process::exit(1);
//   }
// }
// ```
pub fn load_config(file_name: &str) -> Result<Config, Box<dyn Error>> {
    let toml_str = fs::read_to_string(file_name)?;
    let config: Config = toml::from_str(&toml_str)?;
    Ok(config)
}
