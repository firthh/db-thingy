use std::process::Command;
use yaml_rust::Yaml;

pub fn get_cmd (config: &Yaml) -> Command {
    let mut cmd = Command::new("mysql");
    cmd.arg(format!("-h{}", config["host"].as_str().unwrap()))
        .arg(format!("-u{}", config["username"].as_str().unwrap()))
        .arg(format!("--password={}", config["password"].as_str().unwrap()))
        .arg(format!("{}", config["database"].as_str().unwrap()));
    cmd
}

