#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{:#?}", rec1); // Debug 输出格式，需要增加 Debug 的trait

    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );
}
