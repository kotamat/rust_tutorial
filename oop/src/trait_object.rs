pub(crate) fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec!["Yes", "Maybe", "No"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "OK".to_string(),
            }),
        ],
    };
    screen.run();
}

trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Screen2<T: Draw> {
    // Tはtraitでもstructでもあり得るのでdynは使えない。
    // 一方Box<dyn Draw> を使わないと、vec![]で違う型を持てないのでエラー。
    // つまり、このstructでは主題の要求は表現できない
    pub components: Vec<Box<T>>,
}
impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("button drawn");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("select box drawn");
    }
}
