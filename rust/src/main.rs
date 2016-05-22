extern crate yaml_rust;
use yaml_rust::{YamlLoader,Yaml};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;

mod mysql;
mod database;

use mysql::*;
use database::*;

fn open_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => panic!("couldn't read {}", display),
        Ok(_) => println!("Opened configuration file {}", display),
    }
    s
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let environment: &str = match args.get(1) {
        Some(x) => &x,
        None => "production",
    };

    println!("Connecting to Environment {}", environment);

    // Assumes we're in a ruby project
    let s: String = open_file("config/database.yml");

    let docs: Vec<Yaml> = YamlLoader::load_from_str(&s).unwrap();

    let config: &Yaml = &docs[0][environment];

    match config["adapter"].as_str().unwrap() {
        "mysql2" => MySql::new(config).connect(),
        _ => println!("Could find adapter")
    };
}
