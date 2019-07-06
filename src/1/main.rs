/*
 * Rustのスレッド（メッセージ送受信）
 * CreatedAt: 2019-07-06
 */
use std::thread;
use std::sync::mpsc;
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
