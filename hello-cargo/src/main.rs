fn main() {
    let mut a_number = 10; // mut 可变变量
    let a_boolean = true;

    println!("Hello, cargo!");
    println!("the number is {}.", a_number);

    a_number = 15;
    println!("the number is {}.", a_number);
    println!("the boolean is {}", a_boolean);

    println!("------------------------");

    let b_number = 5;      // 阴影操作 “隐藏”
    let b_number = b_number + 5;
    let b_number = b_number * 2;
    println!("The number is {}", b_number); // 20

    println!("-----------数据类型-------------");
    
    let x = 32;
    let y = 3.2;
    println!("{} + {}", x, y);

    println!("-----------数学运算------------");

    println!("1 + 3 = {}", 1u32 + 3); // addition
    println!("1 - 3 = {}", 1i32 - 3); // substraction
    println!("9 / 2 = {}", 9u32 / 2); // integer division
    println!("9 / 2 = {}", 9.0 / 2.0); // float division
    println!("3 * 6 = {}", 3 * 6); // multiplication

    println!("-----------布尔型-------------");

    let is_bigger = 1 > 4;
    println!("{}", is_bigger);

    println!("------------字符和字符串------------");

    let c = "z";
    let z = "ℤ";
    let heart_eyed_cat = "😻";
    println!("{},{},{}", c, z, heart_eyed_cat);

    let mut hello = String::from("Hello, ");
    hello.push('w');
    hello.push_str("orld!");
    println!("{}",hello);

    println!("------------元组------------"); 
    
    let tuple = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

}