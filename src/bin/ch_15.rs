use crate::List::{Cons, Nil};
use RCList::{Cons as RCCons, Nil as RCNil};
use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    println!("{}", *y);

    let name = Box::new(String::from("Rust"));
    hello(&name);

    let con1 = Rc::new(RCCons(5, Rc::new(RCCons(10, Rc::new(RCNil)))));

    let con2 = RCCons(3, Rc::clone(&con1));
    let con3 = RCCons(4, Rc::clone(&con1));

    println!("con1: {:?}, con2: {:?}, con3: {:?}", &con1, &con2, &con3);

    let rf = RefCell::new(5);

    //Interior Mutability Pattern
    *rf.borrow_mut() = 10;

    println!("rf = {}", rf.borrow());

    let val = Rc::new(RefCell::new(3));
    let vala = Rc::clone(&val);
    let valb = Rc::clone(&val);

    *vala.borrow_mut() += 10;

    println!("valb: {}", valb.borrow());

    let weaka = Rc::new(19);
    let weakb: Weak<i8> = Rc::downgrade(&weaka);

    println!("strong: {}", Rc::strong_count(&weaka));
    println!("weak: {}", Rc::weak_count(&weaka));

    let weakv = weakb.upgrade();

    println!("{:?}", weakv);

    CustomSmartPointer {
        data: String::from("First"),
    };

    CustomSmartPointer {
        data: String::from("Second"),
    };

    println!("main function completed");
}

enum List {
    Cons(i8, Box<List>),
    Nil,
}

#[derive(Debug)]
enum RCList {
    Cons(i8, Rc<RCList>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}
