// src/lib.rs

use serde::{Serialize, Deserialize};

pub mod conf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharedData {
    pub config: conf::Conf,
    pub tokens: Vec<String>,
}

/// ErgonomicKeys holds two lists of keys:
/// - `left`: keys for the first letter (typically for the left hand)
/// - `right`: keys for the second letter (typically for the right hand)
///
/// The default implementation uses a space‐delimited string which is split into a vector,
/// eliminating the need to call `.to_string()` individually.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct SelectedKeys {
    pub left: Vec<String>,
    pub right: Vec<String>,
}

impl Default for SelectedKeys {
    fn default() -> Self {
        // Instead of calling .to_string() on each element
        //,use split_whitespace to build the vector.
        let left = "` 1 2 3 4 5 Q W E R T A S D F G Z X C V B"
            .split_whitespace()
            .map(String::from)
            .collect();
        let right = "6 7 8 9 0 - = Y U I O P [ ] H J K L ; ' N M , . /"
            .split_whitespace()
            .map(String::from)
            .collect();
        SelectedKeys { left, right }
    }
}

impl SelectedKeys {
    pub fn new(left: &str, right: &str) -> Self {
        let left = left
            .split_whitespace()
            .map(String::from)
            .collect();
        let right = right
            .split_whitespace()
            .map(String::from)
            .collect();
        SelectedKeys { left, right }
    }
}

/// Generate a list of tokens to fill a grid based on ergonomic key combinations.
///
/// It works in four phases:
///   1. Phase 1 produces tokens with first letter from `left` and second from `right`.
///   2. Phase 2 produces tokens with first letter from `right` and second from `left`.
///   3. Phase 3 produces tokens with both letters from `left` (ignoring duplicates where both letters are identical).
///   4. Phase 4 produces tokens with both letters from `right` (ignoring duplicates).
///
/// The resulting vector is truncated to exactly `total` tokens.
/// Panics if the total number of unique tokens generated is less than `total`.
pub fn generate_token_list(total: usize, keys: &SelectedKeys) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();

    // Phase 1: left x right.
    for l in &keys.left {
        for r in &keys.right {
            tokens.push(format!("{}{}", l, r));
        }
    }

    // Phase 2: right x left.
    for r in &keys.right {
        for l in &keys.left {
            tokens.push(format!("{}{}", r, l));
        }
    }

    // Phase 3: left x left (exclude duplicates like "QQ")
    for i in 0..keys.left.len() {
        for j in 0..keys.left.len() {
            if i != j {
                tokens.push(format!("{}{}", keys.left[i], keys.left[j]));
            }
        }
    }

    // Phase 4: right x right (exclude duplicates)
    for i in 0..keys.right.len() {
        for j in 0..keys.right.len() {
            if i != j {
                tokens.push(format!("{}{}", keys.right[i], keys.right[j]));
            }
        }
    }

    if tokens.len() < total {
        panic!("Grid too large: unable to generate {} unique tokens.", total);
    }
    tokens.truncate(total);
    tokens
}
