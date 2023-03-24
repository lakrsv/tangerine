use crate::Res;
use std::process::Command;

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

impl ExecutableCommand for ShellCommand {
    #[cfg(target_os = "macos")]
    fn execute(&self) -> Res<()> {
        Command::new("sh").args(["-c", &self.command]).output()?;
        Ok(())
    }
    #[cfg(target_os = "windows")]
    fn execute(&self) -> Res<()> {
        Command::new("powershell")
            .args(["/C", &self.command])
            .output()?;
        Ok(())
    }
    #[cfg(target_os = "linux")]
    fn execute(&self) -> Res<()> {
        Command::new("sh").args(["-c", &self.command]).output()?;
        Ok(())
    }
}
