use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("Blocking main thread until spawned thread is complete");

    // Blocks main thread until completed
    handle.join().unwrap();

    let some_vec = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("moved value: {:?}", some_vec);
    });

    // Blocks main thread
    handle2.join().unwrap();

    println!("Hello, world! (from main thread)");
}
