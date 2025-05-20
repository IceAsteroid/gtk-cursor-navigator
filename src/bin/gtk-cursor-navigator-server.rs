#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{Arg, value_parser, Command};
// use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::thread;
use std::path::PathBuf;
use gtk_cursor_navigator::{
    conf::{APP_NAME, CONF_DIR_DEFAULT, CONF_FILE_SUFFIX, STYLE_FILE_SUFFIX,
           LOG_DIR_DEFAULT, PathBufExt, expand_path,},
    generate_token_list, SharedData, SelectedKeys,
};

fn handle_client(mut stream: TcpStream, shared_data: &SharedData) {
    let json = serde_json::to_string(shared_data)
        .expect("Failed to serialize shared data");
    stream.write_all(json.as_bytes())
        .expect("Failed to write shared data to stream");
}

fn main() {
    let name = "gtk-cursor-navigator-server";

    // Set up static configuration paths.
    APP_NAME.set(name).expect("APP_NAME already initialized");
    CONF_DIR_DEFAULT
        .set(PathBuf::from(format!("~/.config/{}", name)))
        .expect("CONF_DIR_DEFAULT already initialized");
    CONF_FILE_SUFFIX.set(".toml").expect("CONF_FILE_SUFFIX already initialized");
    STYLE_FILE_SUFFIX.set(".css").expect("STYLE_FILE_SUFFIX already initialized");
    LOG_DIR_DEFAULT
        .set(PathBuf::from("/tmp/"))
        .expect("LOG_DIR_DEFAULT already initialized");

    // Build the default configuration file path.
    let config_default_static: &'static PathBuf = {
        let config_dir = PathBuf::from("~/.config").join(name);
        let config_file_default = config_dir.join("config.toml");
        Box::leak(Box::new(config_file_default))
    };

    // Use Clap to parse command-line arguments for the server.
    let command = Command::new(name)
        .author("IcyTomato")
        .version("0.1")
        .about("Server for gtk-cursor-navigator")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG")
                .help("Sets the server config file path")
                .value_parser(value_parser!(PathBuf))
                .default_value(
                    config_default_static
                        .to_str()
                        .expect("The default config path must be valid Unicode!")
                ),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Sets the port for the server to listen on")
                .value_parser(value_parser!(u16))
                .default_value("7878"),
        );
    let matches = command.get_matches();

    let config_file = matches.get_one::<PathBuf>("config")
        .expect("The `config` option not found");

    let expanded_config = expand_path(config_file.to_str().unwrap());
    println!("Server using config file: {:?}", expanded_config);

    let config = expanded_config.read_config();
    println!("Server configuration:\n{:#?}", config);

    // Generate token list using the common function.
    let total_cells = (config.grid.rows as usize) * (config.grid.columns as usize);
    // let tokens = generate_token_list(total_cells, &SelectedKeys::default());
    let selected_keys =
        SelectedKeys::new(&config.grid.key_left, &config.grid.key_right);
    let tokens =
        generate_token_list(total_cells, &selected_keys);
    let shared_data = SharedData {
        config,
        tokens,
    };

    let port = *matches.get_one::<u16>("port").unwrap();
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr)
        .expect("Failed to bind TCP listener");
    println!("Server listening on {}", addr);

    // Loop forever, handling clients by spawning a new thread.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let sd = shared_data.clone();
                thread::spawn(move || {
                    handle_client(stream, &sd);
                });
            }
            Err(e) => {
                eprintln!("Error accepting client connection: {:?}", e);
            }
        }
    }
}
