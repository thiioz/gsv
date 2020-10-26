use std::result::Result;

pub fn process() -> Result<(), String> {
    println!("usage: gsv [command] <flags>");
    println!("     - gsv help (display this help)");
    println!("     - gsv version (display gsv version)");
    println!("     - gsv semver (calculate and prompt semantic versionning)");
    println!("           options:");
    println!("                 --repository: Path to the git repository, otherwise use actual directory");
    println!("                 --configuration: Path to the gsv configuration file, by default use gsv.yml in repository root");

    return Ok(());
}