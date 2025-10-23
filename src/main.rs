use std::{env, error::Error, process};
use std::path::Path;
use ignore::WalkBuilder;

fn main() {
    println!("Welcome to repopack!");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    repo_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let repo_path = args[1].clone();

        if !Path::new(&repo_path).is_dir() {
            return Err("The specified path does not exist or is not a directory");
        }

        Ok(Config { repo_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Packing repository located in: {}", config.repo_path);

    let files = get_file_paths(&config.repo_path);

    println!("Found {} files:", files.len());
    for file in files {
        println!("{}", file);
    }

    Ok(())
}

fn get_file_paths(repo_path: &str) -> Vec<String> {
    WalkBuilder::new(repo_path)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().map(|ft| ft.is_file()).unwrap_or(false)
        })
        .map(|e| e.path().display().to_string())
        .collect()
}
