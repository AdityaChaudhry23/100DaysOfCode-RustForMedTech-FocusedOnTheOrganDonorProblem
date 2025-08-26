use std::env;
use std::io::{self, stdin, stdout, Write};
use std::path::Path;
use std::process::{Command, Stdio, Child};

fn main() {
    loop {
        // prompt
        print!("> ");
        stdout().flush().ok();

        // read line
        let mut input = String::new();
        let n = stdin().read_line(&mut input).unwrap_or(0);
        if n == 0 {
            break; // EOF (Ctrl-D)
        }

        // allow both " | " and "|" (and extra spaces)
        let mut commands = input.trim().split('|').peekable();
        let mut previous: Option<Child> = None;

        while let Some(cmd_str) = commands.next() {
            let mut parts = cmd_str.trim().split_whitespace();
            let cmd = match parts.next() {
                Some(c) => c,
                None => continue, // empty segment like "echo hi |   | wc"
            };
            let args: Vec<&str> = parts.collect();

            match cmd {
                "cd" => {
                    let new_dir = args.get(0).copied().unwrap_or("/");
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(root) {
                        eprintln!("cd: {}: {}", new_dir, e);
                    }
                    previous = None; // break any pipeline
                }
                "exit" => return,
                _ => {
                    // stdin for this command: pipe from previous, or inherit
                    let stdin_for_child = if let Some(mut prev_child) = previous.take() {
                        let prev_stdout = prev_child
                            .stdout
                            .take()
                            .expect("previous stdout not piped; cannot connect pipeline");
                        Stdio::from(prev_stdout)
                    } else {
                        Stdio::inherit()
                    };

                    // stdout for this command: piped if there's another stage, else inherit
                    let stdout_for_child = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    match Command::new(cmd)
                        .args(&args)
                        .stdin(stdin_for_child)
                        .stdout(stdout_for_child)
                        .spawn()
                    {
                        Ok(child) => previous = Some(child),
                        Err(e) => {
                            eprintln!("{}: {}", cmd, e);
                            previous = None;
                        }
                    }
                }
            }
        }

        // wait for the last command in the pipeline
        if let Some(mut last) = previous {
            if let Err(e) = last.wait() {
                eprintln!("wait error: {}", e);
            }
        }
    }
}
