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
        println!("{self:#?} is being dropped!");
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
