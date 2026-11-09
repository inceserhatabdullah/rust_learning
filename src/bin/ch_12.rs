use std::env;
use std::process;

use rust_learning::{ArgsConfig, args_run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_config = ArgsConfig::builds(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = args_run(args_config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
