#![allow(dead_code)]

use crate::VectorData::Text;
use rust_learning::User;
use std::collections::HashMap;

#[derive(Debug)]
enum VectorData {
    Boolean(bool),
    Float(f32),
    Text(String),
    Integer(i8),
}

fn main() {
    let mut _users: Vec<User> = Vec::new();

    _users.push(User {
        id: 1,
        name: String::from("name"),
        email: String::from("email"),
    });
    _users.push(User {
        id: 2,
        name: String::from("name 2"),
        email: String::from("email 2"),
    });

    let mut _users_roles: HashMap<u8, String> = HashMap::new();

    _users_roles.insert(1, String::from("admin"));
    _users_roles.insert(2, String::from("user"));

    for user in _users {
        println!(
            "Name: {}, Email: {}, Role: {:?}",
            user.name,
            user.email,
            _users_roles.get(&user.id)
        )
    }
}

fn vectors() {
    let mut vector: Vec<i8> = Vec::new();
    let vector2 = vec![1, 2];

    vector.push(12);

    println!("vector1: {vector:?} vector2: {vector2:?}");

    let v1 = vector.get(1);
    let v2 = vector2[1];

    if let Some(r) = v1 {
        println!("v1.get(0) = {r}")
    } else {
        println!("v1.get(0) = {v1:?}");
    }

    println!("{v2:?}");

    for index in &vector2 {
        println!("{index}")
    }

    for i in &mut vector {
        println!("before change: {i}");
        *i += 1;
        println!("after change: {i}");
    }

    let vector_datas: Vec<VectorData> = vec![
        VectorData::Boolean(false),
        VectorData::Float(0.5),
        VectorData::Integer(1),
        Text(String::from("value")),
    ];
    println!("{vector_datas:#?}");
}

fn strings() {
    let mut empty_string = String::new();

    println!("empty: {empty_string}");

    empty_string.push_str("not empty anymore!");

    println!("empty: {empty_string}");

    let to_string = String::from("selam");
    println!("to_string: {to_string}");

    let format_string = format!("{x} {empty_string}", x = to_string);
    println!("combined: {format_string}");

    let first_char = &empty_string.get(0..1);

    if let Some(first_) = first_char {
        println!("first char: {first_}");
    }

    let first_char2 = &empty_string[0..1];
    println!("first char: {first_char2}");

    for c in empty_string.chars() {
        println!("char: {}", c);
    }
}

fn hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("ef"), 5);
    scores.insert(String::from("aurora"), 8);
    scores.insert(String::from("fut"), 9);

    let score = scores.get(&String::from("aurora1")).copied().unwrap_or(-1);
    println!("aurora1:{}", score);

    for (key, value) in &scores {
        println!("key: {key}, value: {value}");
    }

    let mut map = HashMap::new();

    let text = "world fck sck world";

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map: {map:?}");
}
