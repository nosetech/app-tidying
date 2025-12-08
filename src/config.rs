use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowLayout {
    pub app_name: String,
    pub window_title: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub name: String,
    pub windows: Vec<WindowLayout>,
}

#[allow(dead_code)]
pub fn get_config_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".config/app-tidying")
}

#[allow(dead_code)]
pub fn load_layout(layout_name: &str) -> Result<LayoutConfig, Box<dyn std::error::Error>> {
    let config_dir = get_config_dir();
    let config_path = config_dir.join(format!("{}.json", layout_name));

    let content = fs::read_to_string(&config_path)?;
    let config = serde_json::from_str(&content)?;

    Ok(config)
}

#[allow(dead_code)]
pub fn save_layout(layout: &LayoutConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_config_dir();

    fs::create_dir_all(&config_dir)?;

    let config_path = config_dir.join(format!("{}.json", layout.name));
    let json = serde_json::to_string_pretty(layout)?;
    fs::write(&config_path, json)?;

    Ok(())
}
