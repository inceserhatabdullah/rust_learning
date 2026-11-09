use std::{
    fmt::{self, Display},
    fs,
};

#[derive(Debug)]
pub enum ConditionTypes {
    Number(u8),
    Text(String),
}

#[derive(Debug)]
pub struct User {
    pub id: u8,
    pub name: String,
    pub email: String,
}

impl Drop for User {
    fn drop(&mut self) {
        //println!("{self:#?} is being dropped!");
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, _rectangle: &Rectangle) -> bool {
        self.area() > _rectangle.area()
    }

    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug)]
pub enum Message {
    Write(String),
}

impl Message {
    pub fn call(&self) {
        println!("{self:#?}")
    }
}

#[derive(Debug)]
pub enum State {
    Alabama,
    Alaska,
}

pub enum Coin {
    Quarter(State),
    Penny,
}

impl State {
    pub fn existed_in(&self, year: u16) -> bool {
        match self {
            State::Alabama => year >= 1819,
            State::Alaska => year >= 1959,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub username: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Article {
    pub headline: String,
    pub author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        return format!("{} by {}", self.headline, self.author);
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "display: {}", self.author)
    }
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("{}", item.summarize());
    println!("{}", item);
}

/*
    pub fn notify<T: Summary>(item: &T) {}

    pub fn notify<T>(item: T)
    where
        T: Summary + Clone,
    {
    }

*/

#[derive(Debug)]
pub struct ArgsConfig {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl ArgsConfig {
    pub fn builds(args: &[String]) -> Result<ArgsConfig, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(ArgsConfig {
            query: args[1].clone(),
            filepath: args[2].clone(),
            ignore_case: std::env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn args_run(args_config: ArgsConfig) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(args_config.filepath)?;

    println!("File Full Content: {content}");

    let results = if args_config.ignore_case {
        search_case_insensitive(&args_config.query, &content)
    } else {
        search(&args_config.query, &content)
    };

    for line in results {
        println!("Searched Lines: {line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
safe, fast, productive
            Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
}
