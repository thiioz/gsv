use std::result::Result;
use std::path::Path;

use super::super::utils::{CLI_ARGS, YAML_CONFIGURATION};
use super::super::utils::git_client::{git_version, git_log, read};

pub fn process() -> Result<(), String> {
    let git_version_output = git_log();
    
    if !git_version().status.success() {
        return Err(read(&git_version_output.stderr));
    }

    if !Path::new(&CLI_ARGS.repository_path).join(Path::new(".git")).exists() {
        return Err(format!("{} is not a valid git repository", CLI_ARGS.repository_path));
    }

    let git_log_output = git_log();

    if !git_log_output.status.success() {
        return Err(read(&git_log_output.stderr));
    }

    compute_semver_on(read(&git_log_output.stdout));

    return Ok(());
}

fn compute_semver_on(messages: String) {
    let mut major: u32 = 0;
    let mut minor: u32 = 0;
    let mut patch: u32 = 0;

    for message in messages.split('\n').rev() {
        if YAML_CONFIGURATION.bump_major_regex.is_match(message) {
            major += 1;
            minor = 0;
            patch = 0;
        } else if YAML_CONFIGURATION.bump_minor_regex.is_match(message) {
            minor += 1;
            patch = 0;
        } else if YAML_CONFIGURATION.bump_patch_regex.is_match(message) {
            patch += 1;
        }
    }

    println!("{}.{}.{}", major, minor, patch);
}