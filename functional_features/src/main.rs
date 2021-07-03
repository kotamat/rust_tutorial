mod improve_io;
mod iterators;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;
    //
    // generate_workout(simulated_user_specified_value, simulated_random_number);
    // generate_workout_with_struct(simulated_user_specified_value, simulated_random_number);

    iterators::main();
    improve_io::main();
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        )
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            )
        }
    }
}

fn generate_workout_with_closure(intensity: u32, random_number: u32) {
    // closure
    let expensive_result = |num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result(intensity));
        println!("Next, do {} situps!", expensive_result(intensity))
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result(intensity))
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculationg slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

//  closure with struct
struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash,
{
    calculation: T,
    value: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value.get(&arg) {
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(v, arg);
                v
            }
            Some(v) => *v,
        }
    }
}

fn generate_workout_with_struct(inversity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if inversity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(inversity));
        println!("Next, do {} situps!", expensive_result.value(inversity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(inversity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2)
}

#[test]
fn capture_environment() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(4));
}

// #[test]
// fn capture_environment_with_move() {
//     let x = vec![1, 2, 3];
//     let equals_to_x = move |z| z == x;
//
//     println!("can't use x here: {:?}", x);
//
//     let y = vec![1, 2, 3];
//     assert!(equals_to_x(y));
// }
