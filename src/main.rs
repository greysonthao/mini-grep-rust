use grep_thrice::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Encountered parsing problem: {}", err);
        process::exit(1);
    });

    println!(
        "\nSearching for '{}' in filename '{}'",
        config.query, config.filename
    );

    if let Err(e) = grep_thrice::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
