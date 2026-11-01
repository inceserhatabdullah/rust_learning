use rust_learning::User;

fn main() {
    println!("welcome ch_4");

    let word = String::from("Word!");

    // anonymous
    let result = |param: &str| {
        println!("{param}");
    };

    // give the reference if you just the need display value
    // if you give the value itself, you can not use word after calling result.
    result(&word);

    let (s, len) = calculate(&word);
    println!("{s} {len}, {word}");

    let mut param = String::from("Selam");
    mutable_borrowing(&mut param);
    println!("{param}");

    slice();

    drop();

    longest("selam", "merhaba");
}

fn calculate(param: &str) -> (&str, usize) {
    let len = param.len();

    (param, len)
}

fn mutable_borrowing(param: &mut String) {
    param.push_str(" Hello ");
}

fn slice() {
    let numbers = [10, 20, 30, 40, 50];
    let slice = &numbers[..3];

    println!("{:?}", slice)
}

fn drop() {
    let mut _user = User {
        id: 1,
        name: String::from("serhat"),
        email: String::from("test"),
    };

    dbg!(&_user);
    println!("User created.");

    let is_valid = validate_user(&_user);
    println!("user is valid: {is_valid}");

    update_user_email(&mut _user, "new email");
}

fn validate_user(_user: &User) -> bool {
    !_user.name.is_empty() && !_user.email.is_empty()
}

fn update_user_email(_user: &mut User, email: &str) {
    _user.email = email.to_string();
}

// lifetime annotation <'a> is a parameter of lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
