// #![allow(dead_code)]
// #![allow(unused_variables)]

// TODO List:
// Complete support for:
//   let reserved_right = config.reserved.right as f64; // correction for right-side bar
//   let reserved_top = config.reserved.top as f64; // correction for top-side bar


extern crate gdk4_sys; // For gdk_keyval_to_unicode

mod conf;

use std::env;
use std::net::TcpStream;
use std::io::{Read, BufReader};
use std::path::PathBuf;
use std::process::Command;
use std::char;
use std::rc::Rc;
use std::cell::RefCell;
use clap::{Arg, value_parser, Command as ClapCommand};
use crate::conf::{
    APP_NAME, CONF_DIR_DEFAULT, CONF_FILE_SUFFIX, STYLE_FILE_SUFFIX, LOG_DIR_DEFAULT,
};
// use gio::prelude::*;
use gtk4::{
    prelude::{WidgetExt, GtkWindowExt, GridExt,
    ApplicationExtManual, ApplicationExt},
    Application, ApplicationWindow, CssProvider, EventControllerKey, Grid, Label,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk4_layer_shell::{Edge, Layer, KeyboardMode, LayerShell};
use log::debug;
use glib::{
    translate::IntoGlib,
    Propagation,
};
use gtk_cursor_navigator::SharedData;  // Provided by your lib.rs

/// Connects to the server via TCP (using BufReader) and retrieves the shared data.
fn retrieve_shared_data_from_server(server_addr: &str) -> SharedData {
    let stream = TcpStream::connect(server_addr)
        .expect("Failed to connect to server");
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Failed to read from server");
    debug!("Received raw data: {}", buffer);
    serde_json::from_str(&buffer)
        .expect("Failed to deserialize JSON from server")
}

/// Generates a CSS string from the theme in the configuration.
/// Note: min-width and min-height are fixed to "0px" per your requirements.
fn generate_css_from_theme(theme: &gtk_cursor_navigator::conf::ConfTheme) -> String {
    format!(
        ".label-cell {{
            background-color: {};
            color: {};
            border: {}px solid {};
            padding: 0px;
            font-weight: {};
            font-size: {}px;
            min-width: 0px;
            min-height: 0px;
        }}",
        theme.background_color,
        theme.foreground_color,
        theme.line_pixel,
        theme.line_color,
        theme.font_weight,
        theme.font_size,
    )
}

/// The GTK activation function builds the layer‑shell window with a grid view.
/// Each cell displays its token and its Label widget is saved for later use in determining
/// its on‑screen coordinates. Two key controllers are installed: one for exiting the app
/// (using a configured shortcut) and one for handling token input. When a complete token is
/// typed, the target cell’s Label widget is queried for its position using
/// `translate_coordinates()`. Its center is determined and (after adding a correction factor for
/// your reserved left margin) the coordinates are sent to ydotool as absolute pixel values
/// using the command:
///     ydotool mousemove --absolute -x <X> -y <Y>
fn activate(application: &gtk4::Application, shared_data: SharedData, css_data: String) {
    let window = ApplicationWindow::new(application);
    let config = &shared_data.config;

    window.set_decorated(false);
    window.set_title(Some("Layer Shell Grid Overlay"));
    // window.set_opacity(config.theme.foreground_opacity as f64);
    window.set_opacity(config.theme.opacity as f64);

    // if config.grid.cover_screen {
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
    // } else {
    //     window.set_default_size(config.grid.width as i32, config.grid.height as i32);
    // }

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.set_exclusive_zone(0);

    let provider = CssProvider::new();
    provider.load_from_data(css_data.as_str());
    if let Some(display) = gtk4::gdk::Display::default() {
        // Using the deprecated method as in your original code.
        gtk4::StyleContext::add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // Create and store each cell's Label in a vector.
    let cell_labels: Rc<RefCell<Vec<Label>>> = Rc::new(RefCell::new(Vec::new()));
    let grid = Grid::new();
    grid.set_focusable(true);
    grid.set_row_homogeneous(true);
    grid.set_column_homogeneous(true);
    grid.set_hexpand(true);
    grid.set_vexpand(true);
    grid.set_margin_top(0);
    grid.set_margin_bottom(0);
    grid.set_margin_start(0);
    grid.set_margin_end(0);

    let rows = config.grid.rows as i32;
    let columns = config.grid.columns as i32;
    for row in 0..rows {
        for col in 0..columns {
            let index = (row as usize * (columns as usize)) + (col as usize);
            let token = if index < shared_data.tokens.len() {
                &shared_data.tokens[index]
            } else {
                ""
            };
            let cell_label = Label::new(Some(token));
            cell_label.add_css_class("label-cell");
            grid.attach(&cell_label, col, row, 1, 1);
            cell_labels.borrow_mut().push(cell_label);
        }
    }
    window.set_child(Some(&grid));

    // --- Key Controller for Exit ---
    let key_controller = EventControllerKey::new();
    {
        let exit_key = config.shortcut.exit_key;
        key_controller.connect_key_pressed(move |_controller, keyval, _keycode, _modifiers| {
            let key_u32: u32 = keyval.into_glib();
            if key_u32 == exit_key {
                std::process::exit(0);
            }
            Propagation::Proceed
        });
    }
    window.add_controller(key_controller);

    window.present();
    window.grab_focus();

    let reserved_left = config.reserved.left as f64; // correction for left-side bar
    let reserved_right = config.reserved.right as f64; // correction for right-side bar
    let reserved_top = config.reserved.top as f64; // correction for top-side bar
    let reserved_bottom = config.reserved.bottom as f64; // correction for bottom-side bar

    let input_buffer: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
    let tokens_for_match = shared_data.tokens.clone();
    let cell_labels_for_move = Rc::clone(&cell_labels);

    // Clone the window so it can be used within the closure.
    let win_for_translation = window.clone();

    let key_controller2 = EventControllerKey::new();
    {
        let buffer_cb = Rc::clone(&input_buffer);
        key_controller2.connect_key_pressed(move |_controller, keyval, _keycode, _modifiers| {
            let key_uint: u32 = keyval.into_glib();
            let unicode = unsafe { gdk4_sys::gdk_keyval_to_unicode(key_uint) };
            if unicode != 0 {
                if let Some(ch) = char::from_u32(unicode) {
                    let final_char = if ch.is_alphabetic() { ch.to_ascii_uppercase() } else { ch };
                    buffer_cb.borrow_mut().push(final_char);
                }
            }
            let current_input = buffer_cb.borrow().clone();
            if !current_input.is_empty() {
                let matching: Vec<(usize, String)> = tokens_for_match.iter()
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

                    // Retrieve the corresponding Label widget.
                    let cell_label = cell_labels_for_move.borrow()[index].clone();
                    // Use translate_coordinates with floating-point zero offsets.
                    if let Some((label_x, label_y)) = cell_label.translate_coordinates(&win_for_translation, 0.0, 0.0) {
                        let alloc = cell_label.allocation();
                        // Compute the center of the label.
                        let center_x = label_x + (alloc.width() as f64 / 2.0);
                        let center_y = label_y + (alloc.height() as f64 / 2.0);
                        // Apply a horizontal correction based on reserved_left.
                        let abs_x = center_x + (reserved_left - reserved_right);
                        let abs_y = center_y - (reserved_bottom - reserved_top);
                        // Convert to integer pixel coordinates.
                        let abs_x_int = abs_x.round() as i32;
                        let abs_y_int = abs_y.round() as i32;
                        println!("Moving cursor to: x={} y={}", abs_x_int, abs_y_int);

                        // Use ydotool to move the mouse pointer.
                        let status = Command::new("ydotool")
                            .arg("mousemove")
                            .arg("--absolute")
                            .arg("-x")
                            .arg(format!("{}", abs_x_int))
                            .arg("-y")
                            .arg(format!("{}", abs_y_int))
                            .status()
                            .expect("Failed to execute ydotool");

                        if status.success() {
                            println!("Pointer moved successfully.");
                        } else {
                            println!("ydotool failed with exit code: {:?}", status);
                        }
                        std::process::exit(0);
                    } else {
                        println!("Failed to translate cell label coordinates relative to window.");
                    }
                } else if current_input.len() >= 2 {
                    buffer_cb.borrow_mut().clear();
                }
            }
            Propagation::Proceed
        });
        window.add_controller(key_controller2);
    }
}

fn main() {
    let name = "gtk-cursor-navigator";

    APP_NAME.set(name).expect("APP_NAME already initialized");
    CONF_DIR_DEFAULT
        .set(PathBuf::from(format!("~/.config/{}", name)))
        .expect("CONF_DIR_DEFAULT already initialized");
    CONF_FILE_SUFFIX
        .set(".toml")
        .expect("CONF_FILE_SUFFIX already initialized");
    STYLE_FILE_SUFFIX
        .set(".css")
        .expect("STYLE_FILE_SUFFIX already initialized");
    LOG_DIR_DEFAULT
        .set(PathBuf::from("/tmp/"))
        .expect("LOG_DIR_DEFAULT already initialized");

    let command = ClapCommand::new(name)
        .author("IcyTomato")
        .version("0.1")
        .about("GTK client for retrieving shared configuration and tokens")
        .arg(
            Arg::new("server")
                .short('s')
                .long("server")
                .value_name("SERVER")
                .help("Sets the server address (e.g., 127.0.0.1:7878)")
                .value_parser(value_parser!(String))
                .default_value("127.0.0.1:7878"),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Enable debug logging")
                .action(clap::ArgAction::SetTrue),
        );
    let matches = command.get_matches();

    env_logger::init();
    if matches.get_flag("debug") {
        debug!("Client debug mode enabled");
    }

    let server_addr = matches.get_one::<String>("server").unwrap();
    debug!("Connecting to server at {}", server_addr);
    let shared_data = retrieve_shared_data_from_server(server_addr);
    debug!("Shared data retrieved: {:?}", shared_data);

    let css_data = generate_css_from_theme(&shared_data.config.theme);

    let app = Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());
    app.connect_activate(move |app| {
        activate(app, shared_data.clone(), css_data.clone());
    });
    app.run_with_args(&[env::args().next().unwrap()]);
}
