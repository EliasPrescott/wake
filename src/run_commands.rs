use std::process::Stdio;

use tokio::process::Command;

use crate::{colors::get_primary_color, wake_command::WakeCommand, wrapped_reader::WrapperReader};

pub async fn run_commands(commands: Vec<WakeCommand>, include_info_headers: bool) {
    commands
        .iter()
        .enumerate()
        .for_each(|(task_index, wake_command)| {
            let (std_out_color, _) = (get_primary_color(task_index), 1);
            println!(
                "\x1b[38;5;{}mWaking [{}{} -> {}]\x1b[0m",
                std_out_color,
                if let Some(alias) = &wake_command.alias {
                    alias.to_owned() + ": "
                } else {
                    "".to_owned()
                },
                wake_command.directory,
                wake_command.command,
            );
        });
    println!();

    let child_tasks: Vec<_> = commands
        .into_iter()
        .enumerate()
        .map(|(thread_index, wake_command)| {
            let (std_out_color, std_err_color) = (get_primary_color(thread_index), 1);

            let mut child = Command::new("sh")
                .current_dir(wake_command.directory.clone())
                .arg("-c")
                .arg(wake_command.command.clone())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();

            let child_std_out = child.stdout.take().unwrap();
            let child_std_err = child.stderr.take().unwrap();

            let mut stdout_reader = WrapperReader::new(
                child_std_out,
                if include_info_headers {
                    format!(
                        "\x1b[38;5;{}m[{}{} -> {}]\n",
                        std_out_color,
                        if let Some(alias) = &wake_command.alias {
                            alias.to_owned() + ": "
                        } else {
                            "".to_owned()
                        },
                        wake_command.directory,
                        wake_command.command,
                    )
                } else {
                    format!("\x1b[38;5;{}m", std_out_color)
                }
                .as_str(),
                "\x1b[0m",
            );
            let mut stderr_reader = WrapperReader::new(
                child_std_err,
                if include_info_headers {
                    format!(
                        "\x1b[38;5;{}m[{}{} -> {}]\n",
                        std_err_color,
                        if let Some(alias) = &wake_command.alias {
                            alias.to_owned() + ": "
                        } else {
                            "".to_owned()
                        },
                        wake_command.directory,
                        wake_command.command,
                    )
                } else {
                    format!("\x1b[38;5;{}m", std_err_color)
                }
                .as_str(),
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
                tokio::io::copy(&mut stdout_reader, &mut tokio::io::stdout()).await
            });

            tokio::task::spawn(async move {
                tokio::io::copy(&mut stderr_reader, &mut tokio::io::stderr()).await
            });

            child_poll_task
        })
        .collect();

    for task in child_tasks {
        task.await.unwrap();
    }
}
