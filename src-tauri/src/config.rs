use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub theme: String,
    pub max_history: usize,
    pub auto_create_chat: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "claude-3-opus-20240229".to_string(),
            theme: "light".to_string(),
            max_history: 100,
            auto_create_chat: true, // デフォルトでは自動作成を有効にする
        }
    }
}

pub fn get_config_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let config_dir = app_handle.path().app_config_dir().unwrap();
    fs::create_dir_all(&config_dir).unwrap();
    config_dir.join("config.json")
}

pub fn load_config(app_handle: &tauri::AppHandle) -> Config {
    let config_path = get_config_path(app_handle);
    
    if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(config) => return config,
                Err(e) => eprintln!("Failed to parse config: {}", e),
            },
            Err(e) => eprintln!("Failed to read config file: {}", e),
        }
    }
    
    // デフォルト設定を返す
    let default_config = Config::default();
    save_config(app_handle, &default_config).unwrap();
    default_config
}

pub fn save_config(app_handle: &tauri::AppHandle, config: &Config) -> Result<(), String> {
    let config_path = get_config_path(app_handle);
    
    match serde_json::to_string_pretty(config) {
        Ok(json) => match fs::write(&config_path, json) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write config file: {}", e)),
        },
        Err(e) => Err(format!("Failed to serialize config: {}", e)),
    }
}