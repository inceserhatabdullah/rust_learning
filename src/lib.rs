use std::fmt::{self, Display, write};

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
