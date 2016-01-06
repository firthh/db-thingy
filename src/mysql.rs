use std::process::Command;
use yaml_rust::Yaml;
use database::*;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub struct MySql;

fn add_arg<'t>(cmd: &'t mut Command, arg: &str) -> &'t mut Command {
	cmd.arg(arg);
	cmd
}


impl Database for MySql {
    fn new(config: &Yaml) -> DatabaseCommand {
        let possible_args = map!{
            "host" => "-h{}",
            "username" => "-u{}",
            "user" => "-u{}",
            "password" => "--password={}",
            "database" => "{}"
        };
        let mut cmd = Command::new("mysql");
        match config.as_hash() {
        	Some(h) => {
        		for (k, v) in h.iter() {
        			println!("{} {}", possible_args.get(k.as_str().unwrap()).unwrap(), v.as_str().unwrap());
        		}
        		println!("done")
        	},
        	None => println!("nothing")
        }
        
        add_arg(&mut cmd, &format!("-h{}", config["host"].as_str().unwrap()));
        add_arg(&mut cmd, &format!("-u{}", config["username"].as_str().unwrap()));
        add_arg(&mut cmd, &format!("--password={}", config["password"].as_str().unwrap()));
        add_arg(&mut cmd, &format!("{}", config["database"].as_str().unwrap()));
        DatabaseCommand { cmd: cmd }
    }
}
