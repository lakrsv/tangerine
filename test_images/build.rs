use std::{env, process::Command};

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let cwd = env::var("CARGO_MANIFEST_DIR")?;

    let output = Command::new("docker")
    .arg("build")
    .arg("--file")
    .arg(&format!(
        "{cwd}/src/dockerfiles/simple_tangerine_server.dockerfile"
    ))
    .arg("--force-rm")
    .arg("--tag")
    .arg("simple_tangerine_server:latest")
    .arg(".")
    .output()?;
if !output.status.success() {
    eprintln!("stderr: {}", String::from_utf8(output.stderr)?);
    bail!("unable to build simple_tangerine_server:latest");
}
eprintln!("Built simple_tangerine_server:latest");

    

    // trigger recompilation when dockerfiles are modified
    println!("cargo:rerun-if-changed=src/dockerfiles");
    println!("cargo:rerun-if-changed=.dockerignore");

    Ok(())
}