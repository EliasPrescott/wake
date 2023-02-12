use std::error::Error;

use clap::Parser;

use run_commands::run_commands;
use wake_command::load_commands_from_file;

mod colors;
mod run_commands;
mod wake_command;
mod wrapped_reader;

/// A simple command-line tool for waking up your complex projects and workflows
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = Some(
    "A simple command-line tool for waking up your complex projects and workflows.
Use .wake files to specify directories and commands to start as child processes.

Example .wake file:
    ./api -> dotnet run
    ./front-end -> npm run
    ./logger -> docker compose up"
))]
struct Args {
    /// Path to .wake file
    #[arg(default_value = ".wake")]
    path: String,

    /// Include info headers
    #[arg(short, long, default_value_t = false)]
    info: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let commands = load_commands_from_file(&args.path)?;

    run_commands(commands, args.info).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::wake_command::WakeCommand;

    #[test]
    fn parse_relative_command() {
        assert!(str::parse::<WakeCommand>("./api -> dotnet run").is_ok());
        assert!(str::parse::<WakeCommand>("../../ -> cargo test && cargo build").is_ok());
    }

    #[test]
    fn parse_absolute_command() {
        assert!(str::parse::<WakeCommand>("/usr/bin -> ls").is_ok());
        assert!(str::parse::<WakeCommand>("C:\\ -> ls").is_ok());
    }

    #[test]
    fn parse_alias_command() -> Result<(), Box<dyn Error>> {
        let cmd = str::parse::<WakeCommand>("primary-api: api -> dotnet run")?;
        assert!(cmd.alias.is_some());
        assert!(cmd.alias.unwrap() == "primary-api");
        Ok(())
    }

    #[test]
    fn reject_bad_commands() {
        assert!(str::parse::<WakeCommand>("ls").is_err());
    }
}
