use std::{thread, process::{Command, Stdio}, io::{Read, stdout, Write, stderr}};

use crate::{colors::{DEV_COLOR_LIST, get_primary_color}, wrapped_reader::WrapperReader};

pub fn run_commands(commands: Vec<(String, String)>) {
    let threads: Vec<_> = commands
        .into_iter()
        .enumerate()
        .map(|(thread_index, (directory, command))| {
            let (std_out_color, std_err_color) = (get_primary_color(thread_index), 1);
            println!(
                "\x1b[38;5;{}m{} -> {} -> Success\x1b[0m|\x1b[38;5;{}mError\x1b[0m",
                std_out_color,
                directory,
                command,
                std_err_color,
            );

            thread::spawn(move || {
                let child = Command::new("sh")
                    .current_dir(directory)
                    .arg("-c")
                    .arg(command)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();

                let child_std_out = child.stdout.unwrap();
                let child_std_err = child.stderr.unwrap();

                let mut stdout_reader = WrapperReader::new(
                    child_std_out,
                    &format!("\x1b[38;5;{}m", std_out_color),
                    "\x1b[0m",
                );
                let mut stderr_reader = WrapperReader::new(
                    child_std_err,
                    &format!("\x1b[38;5;{}m", std_err_color),
                    "\x1b[0m",
                );

                let mut buf = vec![0; 8000];

                loop {
                    let out_read = stdout_reader.read(&mut buf).unwrap();
                    if out_read > 0 {
                        let mut std_out = stdout().lock();
                        std_out.write(&mut buf).unwrap();
                        buf = vec![0; 8_000];
                    }

                    let err_read = stderr_reader.read(&mut buf).unwrap();
                    if err_read > 0 {
                        let mut std_err = stderr().lock();
                        std_err.write(&mut buf).unwrap();
                        buf = vec![0; 8_000];
                    }

                    if out_read == 0 && err_read == 0 {
                        break;
                    }
                }
            })
        })
        .collect();
    
    for thread in threads {
        thread.join().unwrap();
    }
}
