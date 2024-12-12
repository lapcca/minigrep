use std::fs;
use std::error::Error;
use std::env;


pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}


impl Config {
    pub fn build(mut args: impl Iterator<Item =String>) -> Result<Config, &'static str>{
        // skip the program name
        args.next();
        let mut query = String::new();
        let mut file_path = String::new();
        let mut ignore_case = env::var("IGNORE_CASE").map_or(false, |s| s.as_str()=="1");
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--ignore-case"| "-i" => {
                    ignore_case = match args.next() {
                            Some(val) if val.as_str().eq("true") => true,
                            Some(val) if val.as_str().eq("false") => false,
                            _ => return Err("No value provided for -i or --ignore-case")
                    };
                },
                "--query" | "-q" => {
                    query = match args.next() {
                        Some(val) => val,
                        None => return Err("No value provided for -q or --query")
                    };
                },
                "--file-path" | "-f" => {
                    file_path = match args.next() {
                        Some(val) => val,
                        None => return Err("No value provided for -f or --file-path")
                    };
                },
                _ => return Err("invalid argument")
            };
        };

        Ok(Config{query, file_path, ignore_case})
    }
}


pub fn run(config:Config)-> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents,Option::from(config.ignore_case)){
        println!("{line}");
    }

    Ok(())
}


pub fn search<'a>(query: &'a str, contents: &'a str, ignore_case: Option<bool>) -> Vec<&'a str>{
    let ignore = ignore_case.unwrap_or(false);
    contents.lines().filter(|line|{
        if ignore{
            line.to_lowercase().contains(&query.to_lowercase())
        }else {
            line.contains(&query)
        }

    }).collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_search_case_sensitive(){
        let query = "hello";
        let contents = "\
hello world
Hello World
Other World
hello there";
        assert_eq!(vec!["hello world", "hello there"], search(query, contents,None));

        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents,None));
    }

    #[test]
    fn test_search_case_insensitive(){
        let query = "hello";
        let contents = "\
hello world
Hello World
Other World
hello there";
        assert_eq!(vec!["hello world","Hello World", "hello there"], search(query, contents,Some(true)));

        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive.","Duct tape."], search(query, contents,Some(true)));
    }
}