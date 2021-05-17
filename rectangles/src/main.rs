#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 定义方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 35,
        height: 48,
    };

    let rec3 = Rectangle {
        width: 28,
        height: 45,
    };

    println!("{:#?}", rec1); // Debug 输出格式，需要增加 Debug 的trait

    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );

    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));
}
