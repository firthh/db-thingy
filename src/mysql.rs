use std::process::Command;
use std::collections::HashMap;
use yaml_rust::Yaml;
use database::*;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m: HashMap<&str, fn(a:&str) -> String> = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub struct MySql;

fn add_arg<'t>(cmd: &'t mut Command, arg: &str) {
	cmd.arg(arg);
}

fn host(h: &str) -> String {
    format!("-h{}", h)
}

fn username(u: &str) -> String {
    format!("-u{}", u)
}

fn password(p: &str) -> String {
    format!("--password={}", p)
}

fn database(d: &str) -> String {
    format!("{}", d)
}

impl Database for MySql {
    fn new(config: &Yaml) -> DatabaseCommand {
        let possible_args = map!{
            "host" =>     host,
            "username" => username,
            "user" =>     username,
            "password" => password,
            "database" => database
        };
        let mut cmd = Command::new("mysql");
        match config.as_hash() {
        	Some(h) => {
        		  for (k, v) in h.iter() {
                  match possible_args.get(k.as_str().unwrap()) {
                      Some(arg) => add_arg(&mut cmd, &arg(v.as_str().unwrap())),
                      None => ()
                  }
        		}
        		()
        	},
        	None => ()
        }
        DatabaseCommand { cmd: cmd }
    }
}
