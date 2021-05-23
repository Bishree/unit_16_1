use std::thread;
use std::time::Duration;

fn main() {
    let handel= thread::spawn(|| {
        for i in 1..10 {
            println!("hi this is {} number from the spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handel.join().unwrap();

    for i in 1..5 {
        println!("hi this is {} umber from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}
