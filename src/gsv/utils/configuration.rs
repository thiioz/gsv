extern crate yaml_rust;

use super::CLI_ARGS;

use std::path::Path;
use regex::Regex;
use std::fs;
use yaml_rust::YamlLoader;

pub struct Configuration {
    pub bump_major_regex: Regex,
    pub bump_minor_regex: Regex,
    pub bump_patch_regex: Regex,
}

pub fn load_configuration_file() -> Configuration {
    let mut configuration = Configuration {
        bump_major_regex: Regex::new("").unwrap(),
        bump_minor_regex: Regex::new("").unwrap(),
        bump_patch_regex: Regex::new("").unwrap()
    };

    if !Path::new(&CLI_ARGS.configuration_path).exists() {
        panic!("Configuration file {} cannot be found", &CLI_ARGS.configuration_path);
    }

    let yaml_content = fs::read_to_string(&CLI_ARGS.configuration_path)
        .expect("Something went wrong reading the file");

    let yaml_document = YamlLoader::load_from_str(&yaml_content)
        .expect("Something went wrong reading the file");

    let doc = &yaml_document[0]["gsv"];

    if doc["bump_major"].is_badvalue() || doc["bump_minor"].is_badvalue() || doc["bump_patch"].is_badvalue() {
        panic!("Configuration file is missing one of the following key: []", );
    }

    let bump_major_str = doc["bump_major"].as_str().unwrap();
    let bump_minor_str = doc["bump_minor"].as_str().unwrap();
    let bump_patch_str = doc["bump_patch"].as_str().unwrap();

    let bump_major_expresion = Regex::new(bump_major_str).unwrap();
    let bump_minor_expresion = Regex::new(bump_minor_str).unwrap();
    let bump_patch_expresion = Regex::new(bump_patch_str).unwrap();

    configuration.bump_major_regex = bump_major_expresion;
    configuration.bump_minor_regex = bump_minor_expresion;
    configuration.bump_patch_regex = bump_patch_expresion;
    
    return configuration;
}