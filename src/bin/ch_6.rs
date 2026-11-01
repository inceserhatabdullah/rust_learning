use rust_learning::{Coin, Message, State};

fn main() {
    let message = Message::Write(String::from("Chapter 6 Hello!"));
    message.call();

    let number: Option<i32> = Some(10);
    let res = number.map(|x| x * 2);

    println!("{res:?}");

    match number {
        Some(n) => println!("{n:?}"),
        None => (),
    }

    let name: Option<String> = Some(String::from("Chapter 6 hi!"));

    let result = name.as_ref().map(|x| {
        println!("{x}");
        x.len()
    });

    println!("{name:?}");
    println!("{result:?}");

    let roll = 9;

    match roll {
        3 => {
            println!("3");
        }
        _ => println!("{roll:?}"),
    }

    let max = Some(3u8);

    if let Some(m) = max {
        println!("max is {m}")
    }

    if let Some(state) = describe(Coin::Quarter(State::Alabama)) {
        println!("{state}");
    };
}

fn describe(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old!"))
    } else {
        Some(format!("{state:?} is relatively new!"))
    }
}
