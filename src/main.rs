#![allow(dead_code)]
#![allow(unused_variables)]

// * Separate server & client
// * Default config path and custom config path
// ** Learn how to generate multiple executables for a Rust project, like in this
// **,case, a server and a client executables.
// * Configuration
// * Needs Fixing: If an option has more than one argument, only the last
// argument will be taken.
// ** Solution 1: Stick with Unix conventions, an option should only take one
// argument.
// ** Solution 2: Correctly takes multiple arguments for an option.
// * Implement regular &  debug logging that can be triggered by cli options.
// * Implement auto testing (for practice).

// * Attach the GPL3 license.

mod cli;
mod conf;

use std::path::PathBuf;
use std::collections::HashMap;

use cli::{ cli_paras_check_valid, CliSameOpts, CliValidParameters, ALLOW_OPTS_ARGS, MUST_PATTERN};
use conf::{APP_NAME, CONF_DIR_DEFAULT, CONF_FILE_SUFFIX, LOG_DIR_DEFAULT, STYLE_FILE_SUFFIX};

fn process_cli_paras() {
    let valid_paras: CliValidParameters =
        CliValidParameters {
            patterns: None,
            opts_args: Some(HashMap::from([
                ("-c".to_string(), Some(vec![String::default()])),
                ("--config".to_string(), Some(vec![String::default()])),
                ("-s".to_string(), Some(vec![String::default()])),
                ("--style".to_string(), Some(vec![String::default()])),
                ("-d".to_string(), None),
                ("--debug".to_string(), None),
                ("-l".to_string(), Some(vec![String::default()])),
                ("--log".to_string(), Some(vec![String::default()])),
            ]))
        };
    let same_opts: CliSameOpts = vec![
        ("-c", "--config"),
        ("-s", "--style"),
        ("-d", "--debug"),
        ("-l", "--log"),
    ];

    let cli_args: Vec<String> = cli::get_raw_cli_paras();
    let parsed_paras: cli::CliParas = cli::cli_paras_parse(&cli_args);
    let parsed_merged_paras = parsed_paras
        .clone()
        .merge_opts_args(&same_opts);
    cli_paras_check_valid(&parsed_paras, &valid_paras);

    if parsed_paras.contains_opt("-c"){
        println!("YES, -f exists");
    }

    println!("cli_args: {:?}", cli_args);
    println!("parsed_paras: {:?}", parsed_paras);
    println!("parsed_merged_paras: {:?}", parsed_merged_paras);
    println!("-f arg: {:?}", parsed_paras.get_args("-f"));
}

fn main() {
    // let conf_dir: PathBuf = PathBuf::from("~/.config/gtk-cursor-navigator/");
    // let conf_toml_path: PathBuf = conf_dir.join("config.toml");
    // println!("conf_toml_path: {:?}", conf_toml_path);

    MUST_PATTERN.set(false)
        .expect("The MUST_PATTEN static should only be set once.");
    ALLOW_OPTS_ARGS.set(true)
        .expect("The ALLOW_OPTS_ARGS static should only be set once.");

    process_cli_paras();

    APP_NAME.set("gtk-cursor-navigator");
    CONF_DIR_DEFAULT.set(PathBuf::from(
        "~/.config/".to_string() + APP_NAME.get().copied().unwrap()));
    CONF_FILE_SUFFIX.set(".toml");
    STYLE_FILE_SUFFIX.set(".css");
    LOG_DIR_DEFAULT.set(PathBuf::from("/tmp/"));
}
