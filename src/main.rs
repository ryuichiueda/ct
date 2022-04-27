//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: GPL-3.0-or-later

use std::io;
use std::collections::HashMap;

fn read_line() -> Option<String> {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    if line.len() > 0 {
        Some((&line.trim_end()).to_string())
    }else{
        None
    }
}

fn main() {
    let mut counter = HashMap::new();

    while let Some(line) = read_line() {
        let count = counter.entry(line).or_insert(0);
        *count += 1;
    };

    for (key, value) in &counter {
        println!("{}: {}", key, value);
    }
}
