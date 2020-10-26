pub mod git_client;

mod configuration;
mod cli;

use cli::{Args, parse_command_line_arguments};
use configuration::{Configuration, load_configuration_file};

lazy_static! {
    pub static ref CLI_ARGS : Args = parse_command_line_arguments();
    pub static ref YAML_CONFIGURATION : Configuration = load_configuration_file();
}
