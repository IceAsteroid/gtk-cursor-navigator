extern crate gdk4_sys; // For gdk_keyval_to_unicode

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, CssProvider, EventControllerKey, Grid, Label,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk4_layer_shell::{Edge, Layer, KeyboardMode, LayerShell};
use std::cell::RefCell;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::process::{Command, Stdio};
use std::io::Write;
use std::char;
use toml;

use gtk4::glib::translate::IntoGlib;
use serde::Deserialize;

// Key constants.
const KEY_ESCAPE: u32 = 0xff1b;
const KEY_Q: u32 = 113;

#[derive(Debug, Deserialize)]
struct Config {
    grid: GridConfig,
    resolution: Resolution,
    reserved: Reserved,
    theme: ThemeConfig,
}

#[derive(Debug, Deserialize)]
struct GridConfig {
    rows: usize,
    columns: usize,
    cover_screen: bool,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct Resolution {
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize)]
struct Reserved {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

#[derive(Debug, Deserialize)]
struct ThemeConfig {
    background_color: String,
    foreground_color: String,
    line_color: String,
    exit_key: String,
    opacity: f64,
}

/// Expand a path that begins with '~' into an absolute path.
fn expand_path(path_str: &str) -> PathBuf {
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

/// Read and parse the configuration file.
fn read_config<P: AsRef<Path>>(path: P) -> Config {
    let config_str = fs::read_to_string(path)
        .expect("Failed to read configuration file");
    toml::from_str(&config_str)
        .expect("Failed to parse configuration file")
}

/// Generate unique two-letter tokens for all grid cells.
/// First, create the primary pool from the two fixed lists:
///   first_set = ["Q", "W", "E", "R", "A", "S", "D", "F", "G"]
///   second_set = ["U", "I", "O", "P", "H", "J", "K", "L", ";"]
/// yielding exactly 81 tokens.
/// If more tokens are needed, then generate the complete set from the
/// extended alphabet:
///   "QWERASDFGUIOPHJKL;BCMNTVXYZ"
/// (27 characters) — total of 729 two-letter tokens — and discard any
/// tokens already in the primary pool.
fn generate_token_list(total: usize) -> Vec<String> {
    let s1 = ["Q", "W", "E", "R", "A", "S", "D", "F", "G"];
    let s2 = ["U", "I", "O", "P", "H", "J", "K", "L", ";"];
    let mut primary: Vec<String> = s1.iter()
        .flat_map(|&x| {
            s2.iter().map(move |&y| format!("{}{}", x, y))
        })
        .collect();
    // If total is within primary capacity, use primary.
    if total <= primary.len() {
        return primary.into_iter().take(total).collect();
    }
    // Else, generate the full set from extended alphabet.
    let extended_alphabet = [
        "Q", "W", "E", "R", "A", "S", "D", "F", "G",
        "U", "I", "O", "P", "H", "J", "K", "L", ";",
        "B", "C", "M", "N", "T", "V", "X", "Y", "Z",
    ];
    let mut extra: Vec<String> = Vec::new();
    for &x in extended_alphabet.iter() {
        for &y in extended_alphabet.iter() {
            let token = format!("{}{}", x, y);
            if !primary.contains(&token) {
                extra.push(token);
            }
        }
    }
    // The full token list is primary followed by extra (in order).
    primary.extend(extra);
    if total > primary.len() {
        panic!("Grid too large: unable to generate unique two-letter tokens for each cell.");
    }
    primary.truncate(total);
    primary
}

fn main() {
    // Default configuration file path.
    let mut config_path = "~/.config/gtk-cursor-navigator/config.toml".to_string();
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--config" || args[i] == "-c" {
            if i + 1 < args.len() {
                config_path = args[i + 1].clone();
                i += 1;
            }
        }
        i += 1;
    }
    let config_path = expand_path(&config_path);
    eprintln!("Using config file: {:?}", config_path);

    let config = read_config(&config_path);
    let num_rows = config.grid.rows;
    let num_columns = config.grid.columns;
    let total_cells = num_rows * num_columns;
    let res_width = config.resolution.width as f64;
    let res_height = config.resolution.height as f64;

    // Generate the token list (will have no duplicates).
    let all_tokens = generate_token_list(total_cells);

    // Shared storages.
    let tokens_rc: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(all_tokens));
    let cell_labels: Rc<RefCell<Vec<Label>>> = Rc::new(RefCell::new(Vec::new()));
    let input_buffer: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

    let app = Application::new(Some("com.example.layergrid"), Default::default());

    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        // Ensure overlay appearance.
        window.set_decorated(false);
        window.set_title(Some("Layer Shell Grid Overlay"));
        window.set_opacity(config.theme.opacity);

        if config.grid.cover_screen {
            window.connect_realize(|win| {
                win.set_anchor(Edge::Top, true);
                win.set_anchor(Edge::Bottom, true);
                win.set_anchor(Edge::Left, true);
                win.set_anchor(Edge::Right, true);
                win.set_margin(Edge::Top, 0);
                win.set_margin(Edge::Bottom, 0);
                win.set_margin(Edge::Left, 0);
                win.set_margin(Edge::Right, 0);
            });
        } else if let (Some(w), Some(h)) = (config.grid.width, config.grid.height) {
            window.set_default_size(w as i32, h as i32);
        } else {
            window.set_default_size(800, 600);
        }

        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_keyboard_mode(KeyboardMode::Exclusive);
        window.set_exclusive_zone(0);

        let css_data = format!(
            ".label-cell {{
                background-color: {};
                color: {};
                border: 1px solid {};
                padding: 10px;
                font-weight: bold;
                font-size: 20px;
                min-width: 40px;
                min-height: 40px;
            }}",
            config.theme.background_color,
            config.theme.foreground_color,
            config.theme.line_color
        );
        let provider = CssProvider::new();
        provider.load_from_data(css_data.as_str());
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::StyleContext::add_provider_for_display(
                &display,
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        // Create a focusable grid.
        let grid = Grid::new();
        grid.set_focusable(true);
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);
        grid.set_margin_top(0);
        grid.set_margin_bottom(0);
        grid.set_margin_start(0);
        grid.set_margin_end(0);

        for row in 0..num_rows {
            for col in 0..num_columns {
                let idx = row * num_columns + col;
                let token = tokens_rc.borrow()[idx].clone();
                let label = Label::new(Some(&token));
                label.add_css_class("label-cell");
                grid.attach(&label, col as i32, row as i32, 1, 1);
                cell_labels.borrow_mut().push(label);
            }
        }

        window.set_child(Some(&grid));
        window.present();
        window.grab_focus();

        let exit_key_val: u32 = match config.theme.exit_key.to_lowercase().as_str() {
            "escape" => KEY_ESCAPE,
            "q" => KEY_Q,
            other => {
                eprintln!("Unknown exit key: {}. Defaulting to escape.", other);
                KEY_ESCAPE
            }
        };

        {
            let tokens_cb = Rc::clone(&tokens_rc);
            let buffer_cb = Rc::clone(&input_buffer);
            let key_controller = EventControllerKey::new();
            key_controller.connect_key_pressed(move |_controller, keyval, _keycode, _modifiers| {
                let key_uint: u32 = keyval.into_glib();
                if key_uint == exit_key_val {
                    println!("Exit key pressed; terminating program.");
                    std::process::exit(0);
                }
                let unicode = unsafe { gdk4_sys::gdk_keyval_to_unicode(key_uint) };
                if unicode != 0 {
                    if let Some(ch) = char::from_u32(unicode) {
                        let final_char = if ch.is_alphabetic() { ch.to_ascii_uppercase() } else { ch };
                        buffer_cb.borrow_mut().push(final_char);
                    }
                }
                let current_input = buffer_cb.borrow().clone();
                if !current_input.is_empty() {
                    let matching: Vec<(usize, String)> = tokens_cb.borrow().iter()
                        .enumerate()
                        .filter_map(|(idx, token)| {
                            if token.starts_with(&current_input) {
                                Some((idx, token.clone()))
                            } else {
                                None
                            }
                        })
                        .collect();
                    if matching.is_empty() {
                        buffer_cb.borrow_mut().clear();
                    } else if matching.len() == 1 && matching[0].1 == current_input {
                        println!("Targeting cell: token {} (cell index {}).", matching[0].1, matching[0].0);
                        let index = matching[0].0;

                        // Reserved area adjustments.
                        let reserved_top = config.reserved.top as f64;
                        let reserved_bottom = config.reserved.bottom as f64;
                        let reserved_left = config.reserved.left as f64;
                        let reserved_right = config.reserved.right as f64;

                        let effective_width = res_width - reserved_left - reserved_right;
                        let effective_height = res_height - reserved_top - reserved_bottom;

                        let cell_width = effective_width / (num_columns as f64);
                        let cell_height = effective_height / (num_rows as f64);
                        let row = index / num_columns;
                        let col = index % num_columns;

                        let effective_center_x = (col as f64 + 0.5) * cell_width;
                        let effective_center_y = (row as f64 + 0.5) * cell_height;

                        let absolute_center_x = reserved_left + effective_center_x;
                        let absolute_center_y = reserved_top + effective_center_y;

                        let pct_x = absolute_center_x / res_width;
                        let pct_y = absolute_center_y / res_height;
                        println!("Calculated percentages: ({:.3}, {:.3})", pct_x, pct_y);

                        // Send command via dotoolc.
                        let mut child = Command::new("dotoolc")
                            .stdin(Stdio::piped())
                            .spawn()
                            .expect("Failed to spawn dotoolc");
                        {
                            let child_stdin = child.stdin.as_mut().expect("Failed to open dotoolc stdin");
                            let cmd = format!("mouseto {:.3} {:.3}\n", pct_x, pct_y);
                            child_stdin.write_all(cmd.as_bytes()).expect("Failed to write to dotoolc stdin");
                        }
                        let status = child.wait().expect("Failed to wait on dotoolc");
                        if status.success() {
                            println!("Pointer moved successfully.");
                        } else {
                            println!("dotoolc failed with exit code: {:?}", status);
                        }
                        std::process::exit(0);
                    } else if current_input.len() >= 2 {
                        buffer_cb.borrow_mut().clear();
                    }
                }
                true.into()
            });
            window.add_controller(key_controller);
        }
    });

    app.run();
}
