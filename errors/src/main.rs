use std::cmp::Ordering;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::io;
use std::panic::panic_any;

fn main() {
    println!("Hello, world!");
    // backtrace();
    // file_read_or_create_closure();
    //
    // file_read_unwrap();

    // let result = read_username_from_file3();
    // println!("the result is: {:?}", result);

    check_custom_type("127.0.0.1".to_string());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e)
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn backtrace() {
    let v = vec![1, 2, 3];
    v[99];
}

fn file_read_or_create() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => {
                panic!("Problemn opening the file: {:?}", other_error)
            }
        }
    };
}

fn file_read_or_create_closure() {
    let file_name = "closure.txt";
    let f = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error)
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
}

fn file_read_unwrap() {
    let f = File::open("unwrap.txt").unwrap();
}

fn check_custom_type(guess: String) {
    let mut secret_number: i32 = 0;
    loop {
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
        match guess.cmp(&secret_number) {
            Ordering::Less => {}
            Ordering::Equal => {}
            Ordering::Greater => {}
        }
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
