use std::{env, error::Error, process};
use std::path::Path;
use repopack::get_file_metadata;

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

    let files = get_file_metadata(&config.repo_path);

    // Print table header
    println!("\n{:<80} {:>12}", "File Path", "Tokens");
    println!("{}", "-".repeat(94));

    for file in &files {
        match file.token_count {
            Some(count) => {
                println!("{:<80} {:>12}", file.path.display(), count);
            }
            None => {
                println!("{:<80} {:>12}", file.path.display(), "unknown");
            }
        }
    }

    // Print summary
    println!("{}", "-".repeat(94));
    let total_tokens: usize = files.iter().filter_map(|f| f.token_count).sum();
    println!("Total files: {} | Total tokens: {}", files.len(), total_tokens);

    Ok(())
}
