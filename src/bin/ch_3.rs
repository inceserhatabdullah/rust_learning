use rust_learning::ConditionTypes;

fn main() {
    println!("welcome ch_3");
}

fn condition_type() {
    let condition = false;
    let value = if condition {
        ConditionTypes::Number(3)
    } else {
        ConditionTypes::Text(String::from("three"))
    };

    println!("Value is {:#?}", value)
}

fn loop_with_counter() -> u8 {
    let mut counter: u8 = 0;

    let result = loop {
        counter += 1;

        if counter == 2 {
            break counter * 12;
        }
    };

    result
}

fn loop_with_label() {
    let mut counter = 0;

    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if counter == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        counter += 1;
    }

    println!("end count = {counter}");
}

fn loop_of_array() {
    let arr = [1, 3, 7, 10];

    println!("array is = {:?}", arr);
    println!("first option is 'for value in &arr'");
    println!("second options is 'for (index, value) in arr.iter().enumerate()'");

    for (index, value) in arr.iter().enumerate() {
        println!("index: {index}, value = {value}");
    }

    for n in (std::ops::Range { start: 1, end: 5 }) {
        println!("n = {n}");
    }
}
