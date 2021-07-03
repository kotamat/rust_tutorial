use std::borrow::Borrow;

mod traits;
mod lifetime;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest2(&char_list);

    println!("The largest char is {}", result);

    traits::main();
    lifetime::main();
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = list[0].borrow();

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
