use std::collections::HashMap;

type CliName = Option<String>;
type CliPattern = Option<String>;
type CliOptsArgs = Option<HashMap<String, Option<String>>>;

#[derive(Debug)]
pub(crate) struct CliParameters {
    name: CliName,
    pattern: CliPattern,
    opts_args: CliOptsArgs,
}

impl CliParameters {
    fn new(name: CliName, pattern: CliPattern, opts_args: CliOptsArgs) -> Self {
        Self { name, pattern, opts_args }
    }
}

trait CliOptsExd {
    fn insert_opt_arg(&mut self, key: String, val: Option<String>);
    fn new(x: CliOptsArgs) -> Self;
}

impl CliOptsExd for CliOptsArgs {
    fn insert_opt_arg(&mut self, key: String, val: Option<String>) {
        match self {
            Some(map) => {
                map.insert(key, val);
            }
            None => {
                let mut map = HashMap::new();
                map.insert(key, val);
                *self = Some(map);
            }
        }
    }

    fn new(x: CliOptsArgs) -> Self {
        x
    }
}

pub(crate) fn parse_cli_paras(cli_paras: &[String]) -> CliParameters {
    let mut cli_paras_1 = CliParameters::new(None, None, None);

    // Return early if empty
    if cli_paras.is_empty() {
        return cli_paras_1;
    }

    // First parameter is the name
    cli_paras_1.name = Some(cli_paras[0].clone());

    // Create an empty opts_args container. Notice we use our trait's associated new() method.
    let mut opts_args: CliOptsArgs = CliOptsArgs::new(None);

    // Instead of `prev_opt: &String`, we keep an owned Option to the last seen option key.
    let mut last_opt: Option<String> = None;

    // Process second argument, if it exists.
    if cli_paras.len() > 1 {
        if !cli_paras[1].starts_with('-') {
            cli_paras_1.pattern = Some(cli_paras[1].clone());
        } else {
            opts_args.insert_opt_arg(cli_paras[1].clone(), None);
            last_opt = Some(cli_paras[1].clone());
        }
    }

    // Process the remainder
    for arg in cli_paras.iter().skip(2) {
        if arg.starts_with('-') {
            opts_args.insert_opt_arg(arg.clone(), None);
            last_opt = Some(arg.clone());
        } else {
            // If it is not an option, it gets attached as a value to the last option key.
            if let Some(ref key) = last_opt {
                opts_args.insert_opt_arg(key.clone(), Some(arg.clone()));
            }
        }
    }

    cli_paras_1.opts_args = opts_args;
    cli_paras_1
}
