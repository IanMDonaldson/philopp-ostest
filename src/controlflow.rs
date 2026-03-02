use std::sync::mpsc;
fn main() {
    let x = (1..=50).fold(0, |sum, item| sum * item);
    let (s,r) = mpsc::channel();
    let handle = thread::spawn(move || {
        println!("hello");
        Ok(())
    });
    handle
}