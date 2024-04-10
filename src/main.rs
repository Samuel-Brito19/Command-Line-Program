use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // não precisa unrawap pois retornará () em caso de sucesso
    if let Err(e) = minigrep::run(config) {
        print!("Aplication error: {e}");
        process::exit(1);
    }
}
