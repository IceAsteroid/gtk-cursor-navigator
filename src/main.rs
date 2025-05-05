#![allow(dead_code)]
#![allow(unused_variables)]

// * Separate server & client
// * Default config path and custom config path
// ** Learn how to generate multiple executables for a Rust project, like in this
// **,case, a server executable and a client executable.
// * Configuration
// * Needs Fixing: If an option has more than one argument, only the last
// argument will be taken.
// ** Solution 1: Stick with Unix conventions, an option should only take one
// argument.
// ** Solution 2: Correctly takes multiple arguments for an option.

use std::env;

mod cli;
mod conf;

fn main() {
    let conf_dir: &str = "~/.config/gtk-cursor-navigator";
    let conf_toml_path: String = conf_dir.to_string() + "/" + "config.toml";

    let cli_args: Vec<String> = env::args().collect();
    let parsed_paras: cli::CliParameters = cli::parse_cli_paras(&cli_args);

    let para_name = parsed_paras.name.clone ();
    let para_pattern = parsed_paras.name.clone();
    // let para_opts_args = parsed_paras.opts_args.clone();
    // let para_opts_args_unwarp = para_opts_args.unwrap_or_default();

    if parsed_paras.contains_opt("-f"){
        println!("YES, -f exists");
    }

    println!("{:?}", parsed_paras);
    println!("{:?}", conf_toml_path);
    println!("-f arg: {:?}", parsed_paras.get_arg("-f"));

    // println!("para_opts_args_unwarp: {:?}", para_opts_args_unwarp);
    // println!("para_opts_args_unwarp: {:?}", para_opts_args_unwarp.is_empty());
}
