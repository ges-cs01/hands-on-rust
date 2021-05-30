use::std::env;
use::std::fs;
use::std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args {}", err);
        process::exit(1);
    });
    
    println!("\n\tSearching for: {}", config.query);
    println!("\tIn file: {}\n", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new (args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 { 
            return Err("necessary param missing");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
