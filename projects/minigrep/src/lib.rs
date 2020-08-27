use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_ok() {
        assert_eq!(
            Config::new0(&vec![
                "progname".to_string(),
                "needle".to_string(),
                "haystack".to_string(),
            ])
            .expect("Failed to create Config"),
            Config {
                query: "needle".to_string(),
                filename: "haystack".to_string(),
                case_sensitive: true,
            }
        );
    }
    #[test]
    #[should_panic]
    fn config_fail() {
        // following should panic because first arg (progname) is missing
        Config::new0(&vec!["needle".to_string(), "haystack".to_string()])
            .expect("Failed to create Config");
    }
    #[test]
    fn run_ok() {
        let config = Config::new0(&vec![
            "progname".to_string(),
            "the".to_string(),
            "/dev/null".to_string(),
        ])
        .expect("Failed to create Config");
        // following should pass because /dev/null exists and can be open/read
        if let Err(e) = run(config) {
            panic!("Failed to run: {}", e);
        }
    }
    #[test]
    #[should_panic]
    fn run_fail() {
        let config = Config::new0(&vec![
            "progname".to_string(),
            "the".to_string(),
            "/dev/notnull".to_string(),
        ])
        .expect("Failed to create Config");
        // following should panic because /dev/notnull doesn't exist
        if let Err(e) = run(config) {
            panic!("Failed to run: {}", e);
        }
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

impl Config {
    // this old code is kept becauses unit tests depend on it
    // how could we synthesize arbitrary env::Args otherwise ?
    #[cfg(test)]
    fn new0(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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
