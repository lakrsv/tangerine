use crate::Res;
use std::process::{Command, Output};

pub trait ExecutableCommand {
    fn execute(&self) -> Res<()>;
}

pub struct ShellCommand {
    command: String,
}

impl ShellCommand {
    pub fn new(command: String) -> Self {
        ShellCommand { command }
    }
}

fn print_output(output: &Output) {
    if !output.stdout.is_empty() {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}

impl ExecutableCommand for ShellCommand {
    #[cfg(target_os = "macos")]
    fn execute(&self) -> Res<()> {
        let output = Command::new("sh").args(["-c", &self.command]).output()?;
        print_output(&output);
        Ok(())
    }
    #[cfg(target_os = "windows")]
    fn execute(&self) -> Res<()> {
        let output = Command::new("powershell")
            .args(["/C", &self.command])
            .output()?;
        print_output(&output);
        Ok(())
    }
    #[cfg(target_os = "linux")]
    fn execute(&self) -> Res<()> {
        let output = Command::new("sh").args(["-c", &self.command]).output()?;
        print_output(&output);
        Ok(())
    }
}
