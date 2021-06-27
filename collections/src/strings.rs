pub fn main() {
    let mut s = "foo".to_string();
    let s2 = "bar";
    // s.push_str("bar");
    s.push_str(s2);
    println!("s: {}", s);

    let s1 = "Hello, ".to_string();
    let s2 = "world".to_string();
    let s3 = s1 + s2.as_ref();
    println!("merged: {}", s3);

    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();
    let s = format!("{}-{}-{}", s1, s2, s3);


    for c in "テストを実装してみる👩‍👩‍👧‍👦".chars() {
        println!("{}", c)
    }
}