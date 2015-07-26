use std::sync::Semaphore;

fn main() {
    println!("Hello, world!");
    let sem = Semaphore::new(3);
    sem.acquire();
    sem.release();
}
