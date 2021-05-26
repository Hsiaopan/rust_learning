use crate::CustomSmartPointer;

#[test]
fn is_dropped() {
    let a = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let b = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.") // 当实例离开作用域 Rust 会自动调用 drop
}

#[test]
fn pre_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created.");

    drop(c); // 在值离开作用域之前调用 std::mem::drop 显式清理
    println!("CustomSmartPointer dropped before the end of main.");
}
