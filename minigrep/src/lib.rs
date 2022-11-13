pub mod mini_grep {
    use std::{
        env::{self, Args},
        error::Error,
        io::Error as IoError,
    };

    pub struct Config {
        query: String,
        filename: String,
        case_sensitive: bool,
    }

    impl Config {
        pub fn new(mut args: Args) -> Result<Config, Box<dyn Error>> {
            args.next();

            let query_result = args.next();
            let query = match query_result {
                Some(arg) => arg,
                None => return Err("Didn't get a query string".into()),
            };

            let filename_result = args.next();
            let filename = match filename_result {
                Some(arg) => arg,
                None => return Err("Didn't get a filename".into()),
            };

            let case_sensitive_result = match env::var("CASE_INSENSITIVE") {
                Ok(val) => val,
                Err(_) => String::from("false"),
            };
            let case_sensitive = match case_sensitive_result.as_str() {
                "true" => true,
                _ => false,
            };

            Ok(Config {
                query,
                filename,
                case_sensitive,
            })
        }
    }

    pub fn run(config: Config) -> Result<(), IoError> {
        let contents_result = std::fs::read_to_string(config.filename);
        let contents = match contents_result {
            Ok(contents) => contents,
            Err(err) => return Err(err),
        };

        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        if results.is_some() {
            for line in results.unwrap() {
                println!("{}", line);
            }
        }

        Ok(())
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Option<Vec<&'a str>> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        Some(results)
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Option<Vec<&'a str>> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        Some(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let results = mini_grep::search(query, contents);
        assert_eq!(results, Some(vec!["safe, fast, productive."]));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let results = mini_grep::search_case_insensitive(query, contents);
        assert_eq!(results, Some(vec!["Rust:", "Trust me."]));
    }
}
