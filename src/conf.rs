use serde::Deserialize;
use std::path::{self, PathBuf};
use std::sync::OnceLock;

pub static APP_NAME: OnceLock<&str> = OnceLock::new();
pub static CONF_DIR_DEFAULT: OnceLock<PathBuf> = OnceLock::new();
pub static CONF_FILE_SUFFIX: OnceLock<&str> = OnceLock::new();
pub static STYLE_FILE_SUFFIX: OnceLock<&str> = OnceLock::new();
pub static LOG_DIR_DEFAULT: OnceLock<PathBuf> = OnceLock::new();

#[derive(Debug, Deserialize)]
struct ConfGrid {
    rows: u8,
    columns: u8,
    width: Option<u32>,
    height: Option<u32>,
    cover_screen: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct ConfTheme {
    background_color: String,
    foreground_color: String,
    line_color: String,
    background_opacity: String,
    foreground_opacity: f32,
}

#[derive(Debug, Deserialize)]
struct ConfShortcut {
    exit_key: String,
}

pub struct AppPaths {
    pub conf_dir: PathBuf,
    pub conf_file: PathBuf,
    pub style_file: PathBuf,
    pub log_dir: PathBuf,
    pub log_file: PathBuf,
}

impl Default for AppPaths {
    fn default() -> Self {
        let conf_dir = CONF_DIR_DEFAULT.get().unwrap();
        let conf_file_suffix = CONF_FILE_SUFFIX.get().copied().unwrap();
        let style_file_suffix = STYLE_FILE_SUFFIX.get().copied().unwrap();
        let log_dir = LOG_DIR_DEFAULT.get().unwrap();
        AppPaths {
            conf_dir: conf_dir.clone(),
            conf_file: conf_dir.join("config".to_string() + conf_file_suffix),
            style_file: conf_dir.join("style".to_string() + style_file_suffix),
            log_dir: log_dir.clone(),
            log_file: log_dir.join("")
        }
    }
}
