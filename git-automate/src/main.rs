use std::process::{Command,exit};
use std::io;

fn update_commit_push(){
    println!("Please Enter a Commit Message :");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message) // Take Message from the User
        .expect("Failed to Read Line");

    // git add -A
    let add_commit = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command");

    if !add_commit.status.success(){
        eprintln!("Error : Failed to add files to git repo");
        exit(1);
    }

    // git commit -M message
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
        .expect("Failed to execute git commit command");

    if !commit_command.status.success(){
        eprintln!("Error : Failed to add files to git repo");
        exit(1);
    }

    // git push origin main
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .expect("Failed to execute git push command");

    if !push_command.status.success(){
        eprintln!("Error : Failed to add files to git repo");
        exit(1);
    }

    println!("Successfully added,committed, and Pushed all Changes.")
}

fn main(){
    update_commit_push();
}