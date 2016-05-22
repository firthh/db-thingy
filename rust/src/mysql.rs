use std::process::Command;
use std::collections::HashMap;
use yaml_rust::Yaml;
use database::*;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m: HashMap<String, Box<Fn(String) -> String>> = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub struct MySql;

impl MySql {
    #[inline]
    pub fn possible_args() -> HashMap<String, Box<Fn(String) -> String>> {
        map!{
            "host".to_string() =>     Box::new(|h: String| format!("-h{}",h) ),
            "username".to_string() => Box::new(|u: String| format!("-u{}",u) ),
            "user".to_string() =>     Box::new(|u: String| format!("-u{}",u) ),
            "password".to_string() => Box::new(|p: String| format!("--password={}",p) ),
            "database".to_string() => Box::new(|d: String| d)
        }
    }
}

impl Database for MySql {
    fn new(config: &Yaml) -> DatabaseCommand {
        let mut cmd = Command::new("mysql");
        match config.as_hash() {
        	  Some(h) => {
        		    for (k, v) in h.iter() {
                    let value: String = v.as_str().unwrap_or("").to_string();
                    MySql::possible_args().get(k.as_str().unwrap())
                        .and_then( |arg: &Box<Fn(String) -> String>| -> Option<String> { cmd.arg(&arg(value)); None });
        		        ()
        	      }
            },
        	None => ()
        }
        DatabaseCommand { cmd: cmd }
    }
}
