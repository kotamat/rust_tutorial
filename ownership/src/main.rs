fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2= {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    run_change();

    // slice
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("the first word is: {}", word);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn run_change() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(s: &mut String) {
    s.push_str(", world")
}

// slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
