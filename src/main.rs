use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    let num1 = Arc::new(Mutex::new(0));
    let num2 = Arc::new(Mutex::new(0));

    let value1a = Arc::clone(&num1);
    let value1b = Arc::clone(&num2);

    let value2a = Arc::clone(&num1);
    let value2b = Arc::clone(&num2);

    let h1 = thread::spawn(move || {
        let mut num1 = value1a.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let mut num2 = value1b.lock().unwrap();

        *num1 += 1;
        *num2 += 1;
    });

    let h2 = thread::spawn(move || {
        let mut num2 = value2b.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let mut num1 = value2a.lock().unwrap();

        *num1 += 1;
        *num2 += 1;
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("Main: end!")
}
