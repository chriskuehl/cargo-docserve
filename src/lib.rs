use std::net::SocketAddr;
use std::process::Command;

mod file_host;


pub struct Config {
    pub addr: SocketAddr,
}

impl Config {
    pub fn run(&self) -> Result<(), String> {
        self.build_docs()?;
        self.host_docs()
    }

    fn build_docs(&self) -> Result<(), String> {
        println!("Building docs...");
        let child = Command::new("cargo")
            .args(&["doc"])
            .status()
            .expect("failed to spawn `cargo doc`");
        match child.code() {
            Some(0) => Ok(()),
            Some(exit_code) => Err(format!("`cargo doc` exited with status code {}", exit_code)),
            None => Err(String::from("`cargo doc` was killed by a signal")),
        }
    }

    fn host_docs(&self) -> Result<(), String> {
        file_host::serve(&self.addr)
    }
}
