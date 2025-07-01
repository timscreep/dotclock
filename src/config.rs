use crate::cli::Args;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClockConfig {
    #[serde(default)]
    pub format: Format,
    #[serde(default = "default_true")]
    pub show_date: bool,
    #[serde(default = "default_true")]
    pub show_time: bool,
    #[serde(default)]
    pub offset: Option<String>,
    #[serde(default = "default_mode")]
    pub mode: String,
}

impl ClockConfig {
    pub fn find_config_path(app_name: &str, file_name: &str) -> Option<PathBuf> {
        let check_path = |base: PathBuf| {
            let path = base.join(app_name).join(file_name);
            if path.exists() { Some(path) } else { None }
        };

        env::var("XDG_CONFIG_HOME")
            .ok()
            .and_then(|dir| check_path(PathBuf::from(dir)))
            .or_else(|| {
                env::var("HOME")
                    .ok()
                    .and_then(|home| check_path(PathBuf::from(home).join(".config")))
            })
    }
    pub fn load_or_create(app_name: &str, file_name: &str) -> Result<Self, String> {
        if let Some(path) = Self::find_config_path(app_name, file_name) {
            match fs::read_to_string(&path) {
                Ok(contents) => {
                    toml::from_str(&contents).map_err(|e| format!("Failed to parse config: {}", e))
                }
                Err(e) => Err(format!("Failed to read config file: {}", e)),
            }
        } else {
            let config_dir = match env::var("XDG_CONFIG_HOME") {
                Ok(dir) => PathBuf::from(dir),
                Err(_) => PathBuf::from(env::var("HOME").expect("HOME not set")).join(".config"),
            }
            .join(app_name);

            if let Err(e) = fs::create_dir_all(&config_dir) {
                return Err(format!("Failed to create config directory: {}", e));
            }

            let config_path = config_dir.join(file_name);
            let default = Self {
                format: Format::default(),
                show_date: true,
                show_time: true,
                offset: None,
                mode: String::from("cli"),
            };

            match toml::to_string_pretty(&default) {
                Ok(toml) => {
                    if let Err(e) = fs::write(&config_path, toml) {
                        return Err(format!("Failed to write default config file: {}", e));
                    }
                    eprintln!("Default config created at {}", config_path.display());
                    Ok(default)
                }
                Err(e) => Err(format!("Failed to serialize default config: {}", e)),
            }
        }
    }

    pub fn merge_args(&self, args: &Args) -> Self {
        Self {
            show_date: args.show_date.unwrap_or(self.show_date),
            show_time: args.show_time.unwrap_or(self.show_time),
            offset: args.offset.clone().or_else(|| self.offset.clone()),
            mode: args.mode.clone().unwrap_or_else(|| self.mode.clone()),
            format: self.format.clone(),
        }
    }
}

fn default_true() -> bool {
    true
}
fn default_mode() -> String {
    "cli".into()
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Format {
    #[serde(default = "default_order", deserialize_with = "deserialize_order")]
    pub order: Vec<String>,
}

fn deserialize_order<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = Vec::<String>::deserialize(deserializer)?;
    if v.is_empty() {
        Ok(default_order())
    } else {
        Ok(v)
    }
}

fn default_order() -> Vec<String> {
    vec![
        "year".into(),
        "month".into(),
        "day".into(),
        "hour".into(),
        "minute".into(),
        "second".into(),
    ]
}
