use std::{env, error::Error, process};
use std::path::Path;
use repopack::get_file_metadata;

fn main() {
    println!("Welcome to repopack!");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
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

    // Print table header
    println!("\n{:<80} {:>12}", "File Path", "Tokens");
    println!("{}", "-".repeat(94));

    let mut file_count = 0;
    let mut total_tokens = 0;

    for file in get_file_metadata(&config.repo_path) {
        let count = file.token_count.map_or("unknown".to_string(), |count| count.to_string());
        println!("{:<80} {:>12}", file.path.display(), count);
        
        file_count += 1;
        if let Some(tokens) = file.token_count {
            total_tokens += tokens;
        }
    }

    // Print summary
    println!("{}", "-".repeat(94));
    println!("Total files: {} | Total tokens: {}", file_count, total_tokens);

    Ok(())
}
