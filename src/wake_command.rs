use std::{error::Error, fmt::Display, str::FromStr};

pub struct WakeCommand {
    pub alias: Option<String>,
    pub directory: String,
    pub command: String,
}

#[derive(Debug)]
pub struct WakeCommandError {
    details: String,
}

impl WakeCommandError {
    pub fn new(details: &dyn ToString) -> Self {
        WakeCommandError {
            details: details.to_string(),
        }
    }
}

impl Display for WakeCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Wake Command Error: {}", self.details))
    }
}

impl Error for WakeCommandError {}

impl FromStr for WakeCommand {
    type Err = WakeCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((directory, raw_command)) = s.split_once("->") {
            if let Some((alias, directory)) = directory.split_once(": ") {
                Ok(Self {
                    alias: Some(alias.trim().to_owned()),
                    directory: directory.trim().to_owned(),
                    command: raw_command.to_owned(),
                })
            } else {
                Ok(Self {
                    alias: None,
                    directory: directory.trim().to_owned(),
                    command: raw_command.to_owned(),
                })
            }
        } else {
            Err(WakeCommandError::new(&format!(
                "Could not parse command entry from file: {}",
                s
            )))
        }
    }
}

pub fn load_commands_from_file(path: &str) -> Result<Vec<WakeCommand>, WakeCommandError> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::parse::<WakeCommand>)
        .collect()
}
