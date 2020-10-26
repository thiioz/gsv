use regex::Regex;

pub struct Args {
    pub command: String,
    pub repository_path: String,
    pub configuration_path: String
}

const REPOSITORY_PATH_FLAG: &str = "--repository";
const CONFIGURATION_FILE_PATH: &str = "--configuration";

pub fn parse_command_line_arguments() -> Args {
    let mut args = Args {
        command: String::from(""),
        repository_path: String::from("."),
        configuration_path: String::from("./gsv.yml")
    };
    
    args = extract_command(args);
    args = extract_flags(args);

    return args;
}

fn extract_command(mut args: Args) -> Args {
    args.command = std::env::args().nth(1).unwrap();
    return args;
}

fn extract_flags(mut args: Args) -> Args {
    let flag_regex = Regex::new(r"--\w*=").unwrap();

    let eligilbe_args: Vec<String> = std::env::args()
        .filter(|arg| flag_regex.is_match(arg))
        .collect();

    for arg in eligilbe_args {
        let vect: Vec<&str> = arg.split("=").collect();
        let key = vect[0];
        let value = vect[1];

        match key {
            REPOSITORY_PATH_FLAG => args.repository_path = value.to_string(),
            CONFIGURATION_FILE_PATH => args.configuration_path = value.to_string(),
            _ => println!("Unknown flag: {}", key)
        }
    }

    return args;
}