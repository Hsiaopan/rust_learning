
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Rectangle{ width: 30, height: 50};

    println!("{:#?}", rec1); // Debug 输出格式，需要增加 Debug 的trait

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rec1)
    );

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


