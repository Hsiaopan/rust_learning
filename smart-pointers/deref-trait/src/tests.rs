use crate::MyBox;

#[test]
fn deref_demo1() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // 通过deref追踪指针的值
}

#[test]
fn deref_demo2() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 在 Box<i32> 上使用deref
}

#[test]
fn deref_demo3() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
