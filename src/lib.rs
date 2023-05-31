use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;


#[cfg(test)]

mod test {
    use super::*;

   #[test]
    fn search_case_sensitive() { 
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["Safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn search_case_insensitive() { 
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
         assert_eq!( 
              vec!["Rust:", "Trust me."],
                 case_insensitive(query, contents)
            )

    }
}

pub fn search<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() { 
        if line.contains(query){ 
            results.push(line);
        }
    }
    results
}

pub fn case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() { 
        if line.to_lowercase().contains(&query){ 
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
pub fn new(args: &[String]) -> Result<Config, &'static str> { 
    if args.len() < 3 { 
        return  Err("Arguments too few!");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config{query, filename, case_sensitive})
}

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { 
    let mut f = File::open(config.filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let result = if config.case_sensitive { 
        search(&config.query, &contents)
    }else { 
        case_insensitive(&config.query, &contents)
    };
    
    for line in result { 
        println!("{}", line);
    }

    Ok(())

}



