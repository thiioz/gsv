mod help;
mod semver;
mod version;

use std::result::Result;

use super::utils::CLI_ARGS;

pub fn process() -> Result<(), String> {
    return match CLI_ARGS.command.as_str() {
        "help" => help::process(),
        "semver" => semver::process(),
        "version" => version::process(),
        _ => help::process()
    }
}