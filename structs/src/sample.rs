#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn make_double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub(crate) fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // tuple
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_of_rect(rect)
    );

    //     struct
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect is {:?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_of_rectangle(&rect)
    );

    //     methods
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    rect.make_double();
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    //     compare
    let rect1 = Rectangle {
        width: 20,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //     call static
    let square = Rectangle::square(10);
    println!("square: {:?}", square);
}

fn area_of_rectangle(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

fn area_of_rect(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}
