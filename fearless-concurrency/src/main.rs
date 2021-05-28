use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn join_handledd() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();  // 主线程会等待，直到新建线程执行完毕之后才开始执行 for 循环，所以输出不会交替出现

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 从 thread::spawn 保存一个 JoinHandle 以确保该进程能够运行至结束
    // 通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。
    // 这两个线程会交替执行。主线程会由于 handle.join() 调用会等待直到新建线程执行完毕
    handle.join().unwrap();  
}
