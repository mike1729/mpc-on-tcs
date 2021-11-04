use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let send = move |tx: mpsc::Sender<String>, s: String| {
        thread::spawn(move || {
            tx.send(s).unwrap();
        });
    };

    send(tx.clone(), String::from("Hello, "));
    send(tx.clone(), String::from("World!\n"));

    while let Ok(msg) = rx.recv() {
        println!("Got: {}", msg);
    }
}
