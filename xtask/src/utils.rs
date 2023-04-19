/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs, io, process::Command};

pub fn cargo(cmd: &str, args: Vec<&str>) -> Result<()> {
    let mut command = Command::new("cargo");
    command.current_dir(project_root());
    command.arg(cmd);
    command.args(args);
    command.status()?;
    Ok(())
}
///
pub fn command(program: &str, args: Vec<&str>) -> Result<()> {
    let mut cmd = Command::new(program);
    cmd.current_dir(project_root());
    cmd.args(args.as_slice()).status()?;
    Ok(())
}
///
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
///
pub fn dist_dir() -> PathBuf {
    project_root().join(".artifacts/dist")
}
///
pub fn execute_bundle(bundle: HashMap<&str, Vec<Vec<&str>>>) -> Result<()> {
    for k in bundle.keys() {
        // Step 1: Rustup
        for i in 0..bundle[k].len() {
            let mut cmd = Command::new(k);
            cmd.current_dir(project_root());
            cmd.args(bundle[k][i].clone().as_slice()).status()?;
        }
    }
    Ok(())
}

pub fn rustup(args: Vec<&str>) -> Result<()> {
    command("rustup", args)
}
/// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

pub struct Cargo {
    queue: Vec<(String, Vec<String>)>,
}

impl Cargo {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }
    pub fn add(&mut self, command: String, args: Vec<String>) {
        self.queue.push((command, args));
    }
    pub fn run(&self) -> Result<()> {
        for (cmd, args) in self.queue.clone() {
            let mut command = Command::new("cargo");
            command.current_dir(project_root());
            command.arg(cmd);
            command.args(args);
            command.status()?;
        }
        Ok(())
    }
}