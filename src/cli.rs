use std::collections::HashMap;

type CliName = Option<String>;
type CliPattern = Option<String>;
<<<<<<< HEAD
type CliOpts = Option<HashMap<String, Option<String>>>;

#[derive(Debug)]
pub(crate) struct CliParameters {
    pub(crate) name: CliName,
    pub(crate) pattern: CliPattern,
    pub(crate) opts_args: CliOpts,
}

impl CliParameters {
    pub(crate) fn new(name: CliName, pattern: CliPattern, opts_args: CliOpts) -> Self {
        Self { name, pattern, opts_args }
    }
    pub(crate) fn contains_name(&self) -> bool {
        self.name.is_some()
    }
    pub(crate) fn contains_pattern(&self) -> bool {
        self.name.is_some()
    }
    pub(crate) fn contains_opt(&self, key: &str) -> bool {
        self.opts_args.as_ref()
            .is_some_and(|map| map.contains_key(key))
    }
    pub(crate) fn contains_arg(&self, key: &str) -> bool {
        if !self.contains_opt(key) { return false }
        self.opts_args.as_ref().unwrap().get(key).is_some()
    }
    pub(crate) fn get_arg(&self, key: &str) -> Option<String> {
        // if !self.contains_arg(key) { return None }
        // Some(self.opts_args.as_ref().unwrap().get(key).clone())
        self.opts_args
            .as_ref()
            .and_then(|map|map.get(key))
            .cloned()
            .flatten()
    }
=======
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
>>>>>>> e46ca67ea4be9a92aa9d40b31281961a42bbf6b2
}

trait CliOptsExd {
    fn insert_opt_arg(&mut self, key: String, val: Option<String>);
<<<<<<< HEAD
    fn new(x: CliOpts) -> Self;
}

impl CliOptsExd for CliOpts {
=======
    fn new(x: CliOptsArgs) -> Self;
}

impl CliOptsExd for CliOptsArgs {
>>>>>>> e46ca67ea4be9a92aa9d40b31281961a42bbf6b2
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
<<<<<<< HEAD
    fn new(x: CliOpts) -> Self {
=======

    fn new(x: CliOptsArgs) -> Self {
>>>>>>> e46ca67ea4be9a92aa9d40b31281961a42bbf6b2
        x
    }
}

pub(crate) fn parse_cli_paras(cli_paras: &[String]) -> CliParameters {
    let mut cli_paras_1 = CliParameters::new(None, None, None);
<<<<<<< HEAD
=======

>>>>>>> e46ca67ea4be9a92aa9d40b31281961a42bbf6b2
    // Return early if empty
    if cli_paras.is_empty() {
        return cli_paras_1;
    }
<<<<<<< HEAD
    // First parameter is the name
    cli_paras_1.name = Some(cli_paras[0].clone());
    // Create an empty opts_args container. Use the trait's associated new() method.
    let mut opts_args: CliOpts = CliOpts::new(None);
    // Instead of `prev_opt: &String`, keep an owned Option to the last seen option key.
    let mut last_opt: Option<String> = None;
=======

    // First parameter is the name
    cli_paras_1.name = Some(cli_paras[0].clone());

    // Create an empty opts_args container. Notice we use our trait's associated new() method.
    let mut opts_args: CliOptsArgs = CliOptsArgs::new(None);

    // Instead of `prev_opt: &String`, we keep an owned Option to the last seen option key.
    let mut last_opt: Option<String> = None;

>>>>>>> e46ca67ea4be9a92aa9d40b31281961a42bbf6b2
    // Process second argument, if it exists.
    if cli_paras.len() > 1 {
        if !cli_paras[1].starts_with('-') {
            cli_paras_1.pattern = Some(cli_paras[1].clone());
        } else {
            opts_args.insert_opt_arg(cli_paras[1].clone(), None);
            last_opt = Some(cli_paras[1].clone());
        }
    }
<<<<<<< HEAD
=======

>>>>>>> e46ca67ea4be9a92aa9d40b31281961a42bbf6b2
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
<<<<<<< HEAD
=======

>>>>>>> e46ca67ea4be9a92aa9d40b31281961a42bbf6b2
    cli_paras_1.opts_args = opts_args;
    cli_paras_1
}
