use serde::{Serialize, Deserialize};
use std::default::Default;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub static APP_NAME: OnceLock<&str> = OnceLock::new();
pub static CONF_DIR_DEFAULT: OnceLock<PathBuf> = OnceLock::new();
pub static CONF_FILE_SUFFIX: OnceLock<&str> = OnceLock::new();
pub static STYLE_FILE_SUFFIX: OnceLock<&str> = OnceLock::new();
pub static LOG_DIR_DEFAULT: OnceLock<PathBuf> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Conf {
    pub grid: ConfGrid,
    pub theme: ConfTheme,
    pub reserved: ReservedNotCovered,
    pub shortcut: ConfShortcut,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ConfGrid {
    pub rows: u8,
    pub columns: u8,
    pub width: u32,
    pub height: u32,
    pub cover_screen: bool,
}

impl Default for ConfGrid {
    fn default() -> Self {
        ConfGrid {
            rows: 30,
            columns: 40,
            width: 1920,
            height: 1080,
            cover_screen: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ConfTheme {
    pub background_color: String,
    pub foreground_color: String,
    pub line_pixel: u8,
    pub line_color: String,
    pub opacity: f32,
    pub background_opacity: f32,
    pub foreground_opacity: f32,
    pub font_weight: String,
    pub font_size: u8,
}

impl Default for ConfTheme {
    fn default() -> Self {
        ConfTheme {
            background_color: "#282c34".to_string(),
            foreground_color: "#abb2bf".to_string(),
            line_pixel: 1,
            line_color: "#56b6c2".to_string(),
            opacity: 0.8,
            background_opacity: 1.0,
            foreground_opacity: 1.0,
            font_weight: "Bold".to_string(),
            font_size: 16,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ReservedNotCovered {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

impl Default for ReservedNotCovered {
    fn default() -> Self {
        ReservedNotCovered {
            top: 0,
            bottom: 0,
            left: 25,
            right: 0,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ConfShortcut {
    pub exit_key: u32,
}

impl Default for ConfShortcut {
    fn default() -> Self {
        ConfShortcut {
            exit_key: 0xff1b, // Escape key numeric value.
        }
    }
}

/// Trait to let a PathBuf read and parse a configuration file.
pub trait PathBufExt {
    fn read_config(&self) -> Conf;
}

impl PathBufExt for PathBuf {
    fn read_config(&self) -> Conf {
        match fs::read_to_string(self) {
            Ok(config_str) => {
                toml::from_str(&config_str).unwrap_or_else(|err| {
                    eprintln!(
                        "Failed to parse configuration file: {}. Using default configuration.",
                        err
                    );
                    Conf::default()
                })
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                eprintln!(
                    "Configuration file {:?} not found. Using default configuration.",
                    self
                );
                Conf::default()
            }
            Err(e) => {
                panic!("#!Failed to read configuration file: {:?}", e);
            }
        }
    }
}

/// Optionally expand a path that begins with '~' into an absolute path.
pub fn expand_path(path_str: &str) -> PathBuf {
    if path_str.starts_with('~') {
        if let Some(home) = env::var_os("HOME") {
            let mut home_path = PathBuf::from(home);
            let stripped = path_str.trim_start_matches('~').trim_start_matches('/');
            home_path.push(stripped);
            home_path
        } else {
            PathBuf::from(path_str)
        }
    } else {
        PathBuf::from(path_str)
    }
}
