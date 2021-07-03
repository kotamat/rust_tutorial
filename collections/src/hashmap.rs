use std::collections::HashMap;
use std::hash::Hash;

pub fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);
    println!("scores : {:?}", scores);

    //     collect
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores : {:?}", scores);

    //     overwrite
    scores.insert("Blue".to_string(), 20);
    println!("overwrote scores : {:?}", scores);

    //     entries
    scores.entry("Yellow".to_string()).or_insert(50);

    println!("average {:?}", average(vec![1, 2, 43, 4]));
    println!("median {:?}", median(vec![1, 2, 43, 4, 5]));
    println!("popular {:?}", popular(vec![1, 2, 2, 43, 4, 4, 4, 5]));
}

fn average(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v.iter() {
        sum += i
    }
    sum / (v.len() as i32)
}

fn median(v: Vec<i32>) -> i32 {
    let mut v = v;
    let median_index = v.len() / 2;
    v.sort();
    v[median_index]
}

fn popular(v: Vec<i32>) -> i32 {
    let mut hash: HashMap<i32, u32> = HashMap::new();
    for i in v.iter() {
        if let Some(value) = hash.get(i) {
            hash.insert(*i, value + 1);
        } else {
            hash.insert(*i, 1);
        }
    }
    let mut max = v[0];
    for i in hash.iter() {
        if hash[&max] < *i.1 {
            max = *i.0;
        }
    }
    max
}
