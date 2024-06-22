use serde::Deserialize;
use std::error::Error;
use std::fs;

/*
 enum
*/

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

// impl<'de> serde::Deserialize<'de> for LogLevel {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         match s.to_lowercase().as_str() {
//             "debug" => Ok(LogLevel::Debug),
//             "info" => Ok(LogLevel::Info),
//             "warn" => Ok(LogLevel::Warn),
//             "error" => Ok(LogLevel::Error),
//             _ => Err(serde::de::Error::custom(format!(
//                 "Invalid log level: {}",
//                 s
//             ))),
//         }
//     }
// }

#[derive(Clone, Debug, Deserialize)]
pub enum JWTKind {
    #[serde(rename = "jwt-simple")]
    JWTSimple,
    #[serde(rename = "jsonwebtoken")]
    JsonWebToken,
    #[serde(rename = "none")]
    None, // meaning disabled
}

// impl<'de> serde::Deserialize<'de> for JWTKind {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         match s.to_lowercase().as_str() {
//             "jwt-simple" => Ok(JWTKind::JWTSimple),
//             "jsonwebtoken" => Ok(JWTKind::JsonWebToken),
//             "none" => Ok(JWTKind::None),
//             _ => Err(serde::de::Error::custom(format!(
//                 "Invalid log level: {}",
//                 s
//             ))),
//         }
//     }
// }

/*
 toml definition
*/

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub server: Server,
    pub jwt: JWT,
    #[allow(dead_code)]
    pub logger: Logger,
    pub db: PostgreSQL,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JWT {
    pub kind: JWTKind,
    pub duration_min: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    #[allow(dead_code)]
    service: String,
    #[allow(dead_code)]
    level: LogLevel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostgreSQL {
    pub enabled: bool,
    pub host: String,
    pub dbname: String,
    pub user: String,
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
