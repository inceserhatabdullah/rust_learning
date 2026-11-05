use rust_learning::{Article, Post, Summary, notify};

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn dot(&self) -> &T {
        &self.x
    }

    fn mixup(self, point: Point<T>) -> Point<T> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}

impl<T: std::fmt::Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("largest number is x = {}", self.x);
        } else {
            println!("largest number is y = {}", self.y);
        }
    }
}

fn main() {
    let point = Point { x: 3, y: 2 };

    println!("p.x: {}", point.dot());

    let point2 = Point { x: 1, y: 5 };
    let point3 = point.mixup(point2);

    point3.cmp_display();

    println!("mixup: {point3:?}");

    let post = Post {
        username: String::from("post"),
        content: String::from("content"),
    };

    println!("1 new post: {}", post.summarize());

    let article = Article {
        author: String::from("author"),
        headline: String::from("headline"),
    };

    notify(&article);
}