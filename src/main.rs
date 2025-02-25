use std::path::PathBuf;

use clap::{command, Parser};
use file::Task;

mod file;
mod action;
mod role;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    cwd: PathBuf,
    relpath: PathBuf,
}

fn main() {
    
    let args = Cli::parse();
    std::env::set_current_dir(args.cwd).unwrap();
    let file = std::fs::read_to_string(&args.relpath).unwrap();
    let playbook: file::Playbook = serde_yaml::from_str(&file).unwrap();
    print_all_playbooks(&args.relpath, &playbook);
    print_all_roles(&playbook);
    //println!("{}", serde_json::to_string_pretty(&playbook).unwrap());
    /* let playbook = file::Playbook::builder()
            .tasks(vec![file::Play::builder()
                .name("test".to_string())
                .tasks(vec![
                    file::Task::builder()
                        .name("test".to_string())
                        .action(file::Action::include_task().file("file.yml".into()).call())
                        .extra_fields(serde_yaml::Value::Null)
                        .build().into(),
                ])
                .extra_fields(serde_yaml::Value::Null)
                .build()])
            .build();
    println!("{:?}", playbook);
    println!("{}", serde_json::to_string_pretty(&playbook).unwrap()); */
}


fn all_playbooks<'a>(path: &'a PathBuf, playbook: &'a file::Playbook) -> Vec<(&'a PathBuf,&'a file::Playbook)> {
    let mut playbooks = vec![(path,playbook)];
    for task in playbook.iter() {
        match &task.action {
            Some(action) => {
                match action {
                    action::Action::IncludeTasks(import) => {
                        playbooks.extend(all_playbooks(&import.file, &import.playbook));
                    },
                    action::Action::ImportPlaybook(import) => {
                        playbooks.extend(all_playbooks(&import.file, &import.playbook));
                    },
                    _ => {}
                }
            },
            None => {}
        }
    }
    playbooks
}

fn print_all_playbooks(path: &PathBuf, playbook: &file::Playbook) {
    for (index,(file, playbook)) in all_playbooks(path, playbook).iter().enumerate() {
        println!("=== Playbook {} at {} ===", index, file.display());
        println!("{}", serde_json::to_string_pretty(playbook).unwrap());
    }
}

fn all_roles(playbook: &Vec<impl AsRef<Task>>) -> Vec<&role::Role> {
    let mut roles = vec![];
    for task in playbook.iter() {
        match &task.as_ref().action {
            Some(action) => {
                match action {
                    action::Action::IncludeTasks(import) => {
                        roles.extend(all_roles(&import.playbook.0));
                    },
                    action::Action::ImportPlaybook(import) => {
                        roles.extend(all_roles(&import.playbook.0));
                    }
                    action::Action::ImportRole(import) => {
                        roles.push(&import.role);
                    },
                    _ => {}
                }
            },
            None => {}
        }
        roles.extend(all_roles(&task.as_ref().tasks));
        roles.extend(all_roles(&task.as_ref().block));
        roles.extend(all_roles(&task.as_ref().handlers));
    }
    roles
}

fn print_all_roles(playbook: &file::Playbook) {
    for (index,role) in all_roles(&playbook.0).iter().enumerate() {
        println!("=== Role {} ===", index);
        println!("{}", serde_json::to_string_pretty(role).unwrap());
    }
}