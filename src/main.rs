use std::{env, process};
use study_rust::{QueryConfig, run};

fn main() {
    let args = env::args().collect();
    let config = QueryConfig::new(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("error when reading file: {:?}", err);
        process::exit(1);
    }
}