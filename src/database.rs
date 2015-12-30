use std::process::Command;
use yaml_rust::Yaml;

pub trait Database {
    fn new(config: &Yaml) -> DatabaseCommand;
}

pub struct DatabaseCommand {
    pub cmd: Command
}

impl DatabaseCommand {
    pub fn connect(&mut self) {
        let mut child = self.cmd
            .spawn()
            .unwrap();

        let ecode = child
            .wait()
            .unwrap();

        match ecode.code() {
            Some(x) => println!("Result: {}", x),
            None    => println!("No exit code")
        }
    }
}
