use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;

fn main() {
    default_generic();
    parameterized_generic();
    same_name_method();
    super_trait();
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn default_generic() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    )
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn parameterized_generic() {
    assert_eq!(Millimeters(10) + Meters(1), Millimeters(1010))
}

// same name methods
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;

impl Dog {
    fn baby_name() -> String {
        "Spot".to_string()
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        "puppy".to_string()
    }
}
fn same_name_method() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());
    // 以下は実行できない（methodではなくfunctionなので）= selfがない
    // println!("A baby dog is called a {}", Animal::baby_name())
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}

fn super_trait() {
    let p = Point { x: 10, y: 40000 };
    p.outline_print();
}

fn sized_type() {
    fn generic<T>(t: T) {
        println!("generic");
    }

    fn generic1<T: Sized>(t: T) {
        println!("generic as Sized")
    }
    fn generic2<T: ?Sized>(t: &T) {
        println!("generic, T may or may not be Sized")
    }

    generic(Point { x: 0, y: 0 });
    generic1(Point { x: 0, y: 0 });
    generic2(&Point { x: 0, y: 0 });
    generic1("hogehoge");
}
