#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::sync::OnceLock;
use std::env;

// type CliOptsArgs = HashMap<String, Option<String>>;
pub type CliOptsArgs = HashMap<String, Option<Vec<String>>>;
pub type CliSameOpts<'a> = Vec<(&'a str, &'a str)>;

// If true, a pattern must be specified in cli.
// pub static mut ALLOW_PATTERN: bool = false;
// pub static mut ALLOW_OPTS_ARGS: bool = false;
pub static MUST_PATTERN: OnceLock<bool> = OnceLock::new();
// If true, options & arguments are allowed to specified in cli.
pub static ALLOW_OPTS_ARGS: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct CliParas {
    pub name: Option<String>,
    pub pattern: Option<String>,
    pub opts_args: Option<CliOptsArgs>,
}

#[derive(Debug)]
pub struct CliValidParameters {
    pub patterns: Option<Vec<String>>,
    pub opts_args: Option<CliOptsArgs>,
}

impl CliParas {
    pub fn new(
        name: Option<String>,
        pattern: Option<String>,
        opts_args: Option<CliOptsArgs>
    ) -> Self {
        Self { name, pattern, opts_args, }
    }
    pub fn contains_name(&self) -> bool {
        self.name.is_some()
    }
    pub fn contains_pattern(&self) -> bool {
        self.name.is_some()
    }
    pub fn contains_opt(&self, key: &str) -> bool {
        self.opts_args
            .as_ref()
            .is_some_and(|map| map.contains_key(key))
    }
    pub fn contains_arg(&self, key: &str) -> bool {
        if !self.contains_opt(key) {
            return false;
        }
        self.opts_args.as_ref().unwrap().get(key).is_some()
    }
    pub fn get_args(&self, key: &str) -> Option<Vec<String>> {
        self.opts_args
            .as_ref()
            .and_then(|map| map.get(key))
            .cloned()
            .flatten()
    }
    pub fn merge_opts_args(mut self, same_opts: &CliSameOpts) -> Self {
        self.opts_args = self.opts_args.cli_paras_merge(same_opts);
        self
    }
}

pub trait CliOptsExd {
    fn insert_opt_arg(&mut self, key: String, val: Option<String>);
    fn cli_paras_merge(&self, same_opts: &CliSameOpts) -> Option<CliOptsArgs>;
    fn new(x: Option<CliOptsArgs>) -> Self;
}

impl CliOptsExd for Option<CliOptsArgs> {
    // Insert a value to the vector of a key.
    fn insert_opt_arg(&mut self, key: String, val: Option<String>) {
        // Lazily initialize the map if it doesn't exist.
        let map = self.get_or_insert_with(HashMap::new);
        match val {
            // When a value is provided.
            Some(v) => {
                map.entry(key)
                    .and_modify(|entry| {
                        match entry {
                            Some(vec) => vec.push(v.clone()),
                            // If the key already exists but with None, replace it with a Vec.
                            entry => *entry = Some(vec![v.clone()]),
                        }
                    })
                    .or_insert(Some(vec![v])); // Insert if the key wasn't present.
            }
            // When no value is provided.
            None => {
                map.entry(key).or_insert(None);
            }
        }
     }
    // Merge arguments of long & short options that are the same.
    // Let the long option has the default one.
    fn cli_paras_merge(&self, same_opts: &CliSameOpts) -> Option<CliOptsArgs> {
        let opts_args = self.as_ref()?;
        let mut merged_opts_args: CliOptsArgs = HashMap::new();
        for &(short, long) in same_opts.iter() {
            let has_short = opts_args.contains_key(short);
            let has_long = opts_args.contains_key(long);
            if has_short || has_long {
                let short_values =
                    opts_args.get(short).cloned().unwrap_or(None);
                let long_values =
                    opts_args.get(long).cloned().unwrap_or(None);
                let merged_values = match (long_values, short_values) {
                    (None, None) => None,
                    (Some(vs), None) | (None, Some(vs)) =>
                        Some(vs),
                    (Some(mut vs1), Some(vs2)) => {
                        vs1.extend(vs2);
                        Some(vs1)
                    }
                };
                merged_opts_args.insert(long.to_string(), merged_values);
            }
        }
        Some(merged_opts_args)
    }
    fn new(x: Option<CliOptsArgs>) -> Self {
        x
    }
}

pub(crate) fn cli_paras_parse(cli_paras: &[String]) -> CliParas {
    let mut cli_paras_1 = CliParas::new(None, None, None);
    // Return early if empty.
    if cli_paras.is_empty() { return cli_paras_1; }
    cli_paras_1.name = Some(cli_paras[0].clone()); // 1st parameter is the name.
    // Create an empty opts_args container.
    let mut opts_args: Option<CliOptsArgs> = Option::<CliOptsArgs>::new(None);
    // Instead of `prev_opt: &String`, keep an owned Option to the last seen option key.
    let mut last_opt: Option<String> = None;
    if cli_paras.len() > 1 { // Process second argument, if it exists.
        // If the seond arg is not prefixed with '-', it's a pattern.
        if !cli_paras[1].starts_with('-') {
            cli_paras_1.pattern = Some(cli_paras[1].clone());
        }
        // Otherwise, it's an option.
        else {
            opts_args.insert_opt_arg(cli_paras[1].clone(), None);
            last_opt = Some(cli_paras[1].clone());
        }
    }
    for arg in cli_paras.iter().skip(2) { // Process the remainder
        // If it's an option, assign it as a key
        if arg.starts_with('-') {
            opts_args.insert_opt_arg(arg.clone(), None);
            last_opt = Some(arg.clone());
        }
        // If it's a non-option, assgin it as a value to the last key.
        else if let Some(ref key) = last_opt {
            let entry = opts_args
                .as_mut()
                .unwrap()
                .entry(key.clone())
                .or_insert(None);
            match entry {
                Some(vec) => vec.push(arg.clone()),
                None => *entry = Some(vec![arg.clone()]),
            }
        }
    }
    cli_paras_1.opts_args = opts_args;
    cli_paras_1
}

pub fn cli_paras_check_valid(cli_paras: &CliParas, valid_paras: &CliValidParameters) {
    let cli_pattern = &cli_paras.pattern;
    let cli_opts_args = &cli_paras.opts_args;
    let valid_patterns = &valid_paras.patterns;
    let valid_opts_args = &valid_paras.opts_args;

    if MUST_PATTERN.get().copied().unwrap() && cli_pattern.is_none() {
        panic!("#!Program needs a pattern, but not specified!")
    }
    if !MUST_PATTERN.get().copied().unwrap() && cli_pattern.is_some() {
        panic!("#!Program doesn't accept a pattern, but gets one!")
    }
    if !ALLOW_OPTS_ARGS.get().copied().unwrap() && cli_opts_args.is_some() {
        panic!("#!Program doesn't allow options & arguments , but specified!")
    }
}

pub fn get_raw_cli_paras() -> Vec<String> {
    env::args().collect()
}
