use clap::Parser;

use load_commands::load_commands_from_file;
use run_commands::run_commands;

mod colors;
mod load_commands;
mod run_commands;
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
async fn main() {
    let args = Args::parse();

    run_commands(load_commands_from_file(&args.path), args.info).await;
}
