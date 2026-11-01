use rust_learning::Rectangle;

fn main() {
    let _rectangle1 = Rectangle {
        width: 3,
        height: 15,
    };

    let _rectangle2 = Rectangle {
        width: 3,
        height: 10,
    };

    println!("Area is {}", _rectangle1.area());
    println!(
        "can rect1 hold rect2? {:#?}",
        _rectangle1.can_hold(&_rectangle2)
    );
    println!("new {:#?}", Rectangle::new(1, 3))
}
