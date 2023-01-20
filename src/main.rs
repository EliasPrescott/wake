use std::env::args;

use run_commands::run_commands;

mod wrapped_reader;
mod colors;
mod run_commands;

fn main() {
    let args: Vec<String> = args().collect();

    let command_file_path =
        match &args[..] {
            // Look for a .wake file in the current directory if there are no command-line args
            [_] => ".wake",
            // If there is one command-line arg, use it as a path to the command file to run
            [_, path] => path,
            _ => panic!("Unhandled number of command-line arguments"),
        };

    run_commands(load_commands_from_file(command_file_path))
}

fn load_commands_from_file(path: &str) -> Vec<(String, String)> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            if let Some((directory, raw_command)) = line.split_once("->") {
                (directory.trim().to_owned(), raw_command.trim().to_owned())
            } else {
                panic!("Could not parse command entry from file: {}", line);
            }
        })
        .collect()
}
