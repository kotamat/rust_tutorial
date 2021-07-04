struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}!", self.data)
    }
}
pub(crate) fn main() {
    let c = CustomSmartPointer {
        data: "my stuff".to_string(),
    };

    let d = CustomSmartPointer {
        data: "other stuff".to_string(),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
