extern crate toml;
use toml::Table;

use std::fs::File;
use std::env;
use std::io::prelude::*;

pub fn load_config() -> Table {
    let mut input = String::new();
    let config_file = env::current_dir().unwrap().join("config.toml");

    File::open(&config_file).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    let mut parser = toml::Parser::new(&input);
    let toml = match parser.parse() {
        Some(toml) => toml,
        None => {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{:?}:{}:{}-{}:{} error: {}",
                         config_file, loline, locol, hiline, hicol, err.desc);
            }
            Table::new()
        }
    };
    return toml;
}
