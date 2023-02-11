pub fn load_commands_from_file(path: &str) -> Vec<(String, String)> {
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
