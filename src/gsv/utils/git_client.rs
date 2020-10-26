use super::CLI_ARGS;
use std::process::{Command, Output};

pub fn git_version() -> Output {
    return Command::new("git").arg("--version")
        .output()
        .expect("Execution of 'git --version' fail please check you're git installation !");
}

pub fn git_log() -> Output {
    return Command::new("git")
        .current_dir(&CLI_ARGS.repository_path)
        .args(&["log", "--format=\"%C(auto) %h %s\""])
        .output()
        .expect("Execution of 'git log --format=\"%C(auto) %h %s\"' fail please check you're git repository !");
}

pub fn read(vec: &std::vec::Vec<u8>) -> String {
    return String::from_utf8_lossy(vec).to_string();
}