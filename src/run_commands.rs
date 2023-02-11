use std::process::Stdio;

use tokio::process::Command;

use crate::{colors::get_primary_color, wrapped_reader::WrapperReader};

pub async fn run_commands(commands: Vec<(String, String)>) {
    let child_tasks: Vec<_> = commands
        .into_iter()
        .enumerate()
        .map(|(thread_index, (directory, command))| {
            let (std_out_color, std_err_color) = (get_primary_color(thread_index), 1);
            println!(
                "\x1b[38;5;{}m{} -> {} -> Success\x1b[0m|\x1b[38;5;{}mError\x1b[0m",
                std_out_color, directory, command, std_err_color,
            );
            (thread_index, (directory, command))
        })
        .map(|(thread_index, (directory, command))| {
            let (std_out_color, std_err_color) = (get_primary_color(thread_index), 1);

            let mut child = Command::new("sh")
                .current_dir(directory)
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();

            let child_std_out = child.stdout.take().unwrap();
            let child_std_err = child.stderr.take().unwrap();

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

            let child_poll_task = tokio::task::spawn(async move {
                loop {
                    let status = child.wait().await.unwrap();
                    if status.success() {
                        break;
                    }
                }
            });

            tokio::task::spawn(async move {
                tokio::io::copy(&mut stdout_reader, &mut tokio::io::stdout())
                    .await
                    .unwrap();
            });

            tokio::task::spawn(async move {
                tokio::io::copy(&mut stderr_reader, &mut tokio::io::stderr())
                    .await
                    .unwrap();
            });

            child_poll_task

            // std::io::copy seems to perform better, especially with long-running-test.sh,
            // but it may have a higher chance of collisions because it is not strictly locking
            // stdout or stderr when it writes.
            // Also I am concerned that copy() is a strictly blocking call, which might mean that
            // the stderr copies will be delayed until stdout is finished
            // std::io::copy(&mut stdout_reader, &mut stdout()).unwrap();
            // std::io::copy(&mut stderr_reader, &mut stderr()).unwrap();
        })
        .collect();

    for task in child_tasks {
        task.await.unwrap();
    }
}
