mod strings;
mod hashmap;

fn main() {
    // vector
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("vector {:?}", v);
    let v = vec![1, 2, 3];
    println!("vector {:?}", v);

//     read vector
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        None => println!("There is no third element"),
        Some(third) => println!("The third element is {}", third)
    }

    if let Some(elem) = v.get(100) {
        println!("The element is {}", elem)
    } else {
        println!("There is no element ")
    }

    //     vector and enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.12),
        SpreadsheetCell::Text("blue".to_string())
    ];

//     string
    strings::main();
//     hashmap
    hashmap::main();
}

