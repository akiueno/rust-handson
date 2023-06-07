use std::thread;
use std::time::Duration;
fn main() {
    thread::spawn(|| {
        for n in 1..10 {
            println!("Thread: No, {}.", n);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for n in 1..10 {
        println!("Main: No, {}.", n);
        thread::sleep(Duration::from_millis(100));
    }
}
