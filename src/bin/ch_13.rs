use std::io::Error;

fn main() {
    let multiplier = 3;

    let multiply = |x| x * multiplier;

    println!("{}", multiply(5));

    let numbers = vec![1, 3, 5, 8];
    let double_numbers: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    // immutable for iter -> &i32
    // ownership does not change, items does not move or change
    let mut iter = numbers.iter();

    println!("{:?}", iter.next());
    println!("{:?}", double_numbers);

    let mut numbers2 = vec![2, 3, 6, 7];

    // mutable for iter_mut -> &mut i32
    // ownership does not change, items does not move, can change.
    // collection must be mut
    for number in numbers2.iter_mut() {
        *number += 2;
    }

    println!("{:?}", numbers2);

    let number3 = vec![0, 1, 5];

    // ownership of the collection passes to the iterator
    // items can be moved. not use the original collection aynmore.
    for number in number3.into_iter() {
        println!("{}", number);
    }

    let even_numbers: Vec<i32> = numbers2
        .iter()
        .filter(|number| **number % 2 == 0)
        .copied()
        .collect();

    println!("{:?}", even_numbers);

    let names = vec!["mustafa", "kemal"];

    for (index, name) in names.iter().enumerate() {
        println!("{index}: {name}");
    }

    quest(&mut vec![1, 2, 10, 13, 0]);

    other()
}

fn quest(items: &mut Vec<i32>) {
    /* items.iter_mut().for_each(|x| {
        println!("{x}", x = *x * 2);
    }); */

    let multiply: Vec<i32> = items.iter_mut().map(|x| *x * 2).collect();
    println!("{multiply:?}");

    let filtered: Vec<i32> = items.iter().filter(|x| **x > 10).copied().collect();
    println!("{filtered:?}");
}

fn other() {
    let numbers = vec![1, 3, 5];
    let names = vec!["mustafa", "kemal"];

    let sum: i32 = numbers.iter().sum();

    let found = numbers.iter().find(|&&x| x > 2).copied();

    let any = numbers.iter().any(|x| x % 2 == 0);
    let all = numbers.iter().all(|x| x % 2 == 0);

    let found_content = found.unwrap_or_default();

    let zip: Vec<_> = numbers.iter().zip(names.iter()).collect();

    println!(
        "sum: {sum}, found: {found:?}, found_content: {found_content}, any: {any}, all: {all}, zip: {zip:?} "
    );
}

fn find() -> Option<i32> {
    let numbers = vec![1, 3, 5];
    numbers.iter().find(|&&x| x > 2).copied()
}

/* fn find<'a>(numbers: &'a [i32]) -> Option<&'a i32> {
    numbers.iter().find(|&&x| x > 2)
}
 */
