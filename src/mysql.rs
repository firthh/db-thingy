use std::process::Command;
use std::collections::HashMap;
use yaml_rust::Yaml;
use database::*;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m: HashMap<&str, Box<Fn(String) -> String + 'static>> = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub struct MySql;

impl Database for MySql {
    fn new(config: &Yaml) -> DatabaseCommand {
        let possible_args = map!{
            "host" =>     Box::new(move |h: String| format!("-h{}",h) ),
            "username" => Box::new(move |u: String| format!("-u{}",u) ),
            "user" =>     Box::new(move |u: String| format!("-u{}",u) ),
            "password" => Box::new(move |p: String| format!("--password={}",p) ),
            "database" => Box::new(move |d: String| d)
        };
        let mut cmd = Command::new("mysql");
        match config.as_hash() {
        	  Some(h) => {
        		    for (k, v) in h.iter() {
                    let value: String = v.as_str().unwrap_or("").to_string();
                    possible_args.get(k.as_str().unwrap())
                        .and_then( |arg: &Box<Fn(String) -> String + 'static>| -> Option<String> { cmd.arg(&arg(value)); None });
        		        ()
        	      }
            },
        	None => ()
        }
        DatabaseCommand { cmd: cmd }
    }
}
