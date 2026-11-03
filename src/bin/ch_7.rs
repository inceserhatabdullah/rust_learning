use database::{DatabaseError, User};

#[path = "../modules/collection.rs"]
mod collections;

fn main() {
    let err = DatabaseError::ConnectionError(String::from("connection failed!"));

    println!("{err:#?}");

    let mut _user = User {
        id: 1,
        name: String::from("serhat"),
    };
    let _user2 = _user.clone();

    println!("{_user:#?} {_user2:#?}");

    collections::print();
}
