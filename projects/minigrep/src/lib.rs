use std::fs;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_ok() {
        assert_eq!(
            Config::new(&vec!["progname".to_string(), "needle".to_string(), "haystack".to_string()]).expect("Failed to create Config"),
            Config{query: "needle".to_string(), filename: "haystack".to_string()}
        );
    }
    #[test]
    #[should_panic]
    fn config_fail() {
        // following should panic because first arg (progname) is missing
        Config::new(&vec!["needle".to_string(), "haystack".to_string()]).expect("Failed to create Config");
    }
    #[test]
    fn run_ok() {
        let config = Config::new(&vec!["progname".to_string(), "the".to_string(), "/dev/null".to_string()]).expect("Failed to create Config");
        // following should pass because /dev/null exists and can be open/read
        if let Err(e) = run(config) {
            panic!("Failed to run: {}", e);
        }
    }
    #[test]
    #[should_panic]
    fn run_fail() {
        let config = Config::new(&vec!["progname".to_string(), "the".to_string(), "/dev/notnull".to_string()]).expect("Failed to create Config");
        // following should panic because /dev/notnull doesn't exist
        if let Err(e) = run(config) {
            panic!("Failed to run: {}", e);
        }
    }
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}
