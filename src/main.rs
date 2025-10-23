use std::{env, process};

fn main() {
    println!("Welcome to repopack!");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });

    println!("Packing repository located in: {}", config.file_path);
}

struct Config {
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
