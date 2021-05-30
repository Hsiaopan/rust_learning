use std::sync::{Arc, Mutex};
use std::thread;

/// RefCell<T> / Rc<T> 与 Mutex<T> / Arc<T> 相似，
// 互斥器（mutex）。任意时刻，其只允许一个线程访问某些数据，
// 为了访问互斥器中的数据，线程寿险需要获取互斥器的锁来表明其希望访问数据。
// 锁是一个座位互斥器一部分的数据结构，它记录谁有数据的排他访问权。
#[test]
fn is_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// 原子引用计算
#[test]
fn is_shared() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
