use std::{env, process};
use minigrep::Config;
use minigrep::run;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("In File: {}", config.file_path);
    if let Err(e) = run(config){
        println!("application is Error because: {}",e);
        process::exit(1);
    }

}

