use std::fmt::Display;

use crate::largest;

pub(crate) fn main() {
    generic_lifetimes_in_functions();
    lifetime_annotations_restrict();
    lifetime_annotations_in();
    lifetime_annotations_with_struct();
}

// fn borrow_live_long() {
//     {
//         let r;
//         {
//             let x = 5;
//             r = &x;
//         }
//
//         println!("r: {}", r)
//     }
// }

fn generic_lifetimes_in_functions() {
    let string1 = "abcd".to_string();
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// &str→&'a strとすることで、明示的なlifetimeを定義できる。&'aはどれも同じlifetime
// 渡ってきた引数のうち、一番lifetimeが短いものがすべての&'aのlifetimeになる。
// ここでは引数、返り値それぞれ同じlifetimeが指定されているため、引数のライフタイムが一番短いものを超えるライフタイムを返り値は持てない
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_annotations_restrict() {
    let string1 = "long string is long".to_string();
    {
        let string2 = "xyz".to_string();
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result)
    }
}

// resultはstring2よりも長くlifetimeを持てないのでコンパイルエラー
// fn lifetime_annotations_over() {
//     let string1= "long string is long".to_string();
//     let result;
//     {
//         let string2 = "xyz".to_string();
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result)
// }

fn pick1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// resultはstring1と同じlifetimeなので問題ない
fn lifetime_annotations_in() {
    let string1 = "long string is long".to_string();
    let result;
    {
        let string2 = "xyz".to_string();
        result = pick1(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result)
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_annotations_with_struct() {
    let novel = "Call me Ishmael. Some years ago...".to_string();
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("important excerpt is {:?}", i)
}

// lifetime elision
// 明示的なアノテーションがなくても参照渡しができる

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
// 条件3つ
// 1.すべての引数がそれぞれ別のlifetime annotationを持っている
//  NG hoge<'a> (x: &'a str, y: &'a str)-> &'a str {}

// 2.引数が一つの場合、返り値のlifetime annotationと同一
//  OK hoge<'a> (x: &'a str)-> &'a str {}

// 3. 複数の引数がある場合、一つは &selfもしくは&mut selfであるとき、この場合は返り値のlifetimeはselfと同様になる
// 　この場合はメソッドになるはず(関数ではない)

impl<'a> ImportantExcerpt<'a> {
    // 条件1
    fn level(&self) -> i32 {
        3
    }

    //     条件3
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// all in

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
