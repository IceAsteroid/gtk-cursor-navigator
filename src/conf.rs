use serde::Deserialize;

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
