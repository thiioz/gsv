#[macro_use]
extern crate lazy_static;

extern crate exitcode;

mod gsv;

use gsv::process::process;

fn main() {
    let result = process();
    
    match result {
        Ok(_) => {
            std::process::exit(exitcode::OK);
        }
        Err(e) => {
            println!("💥🤯💥 Ho no, something terrible happened: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}
