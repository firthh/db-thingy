extern crate yaml_rust;
use yaml_rust::YamlLoader;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;

mod mysql;

fn open_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
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

    // Assumes we're in a ruby project
    let s: String = open_file("config/database.yml");

    let docs = YamlLoader::load_from_str(&s).unwrap();

    let doc = &docs[0];

    println!("Connecting to Environment {}", environment);

    let config = &doc[environment];

    // Assumes the adapter is MySql
    let mut child = mysql::get_cmd(config)
        .spawn()
        .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });

    let ecode = child
        .wait()
        .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });

    match ecode.code() {
        Some(x) => println!("Result: {}", x),
        None    => println!("No exit code")
    }
}
