use std::{env, fs};
use std::error::Error;

pub struct QueryConfig {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl QueryConfig {
    pub fn new(args: Vec<String>) -> Result<QueryConfig, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }
        let query = args.get(1).expect("").clone();
        let filename = args.get(2).expect("").clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(QueryConfig {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: QueryConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
SAFE, FAST, PRODUCTIVE.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
SAFE, FAST, PRODUCTIVE.";

        assert_eq!(
            vec![
                "safe, fast, productive.",
                "SAFE, FAST, PRODUCTIVE.",
            ],
            search_case_insensitive(query, content)
        );
    }
}