use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_insensitve: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let _case_ins = !env::var("CASE_INSENSITIVE").is_err();
        if args.len() < 3 {
            return Err("Not enough args");
        }
        Ok(Config { query: args[1].clone(), file_path: args[2].clone(), case_insensitve: _case_ins })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut out: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            out.push(line);
        }
    }
    out
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut out: Vec<&str> = vec![];
    let _q = query.to_lowercase();
    let _q = _q.as_str();
    for line in contents.lines() {
        if line.to_lowercase().contains(_q) {
            out.push(line);
        }
    }
    out
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(cfg.file_path)?;
    let rslt = if cfg.case_insensitve == false {
        search(&cfg.query, &file_contents)
    } else {
        search_case_insensitive(&cfg.query, &file_contents)
    };
    for line in rslt {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn test_case_insensitive() {
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
