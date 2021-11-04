use std::thread;
use std::time::Duration;

fn io_bound(delay: u64, msg: String) {
    thread::sleep(Duration::from_millis(delay));

    print!("{}", msg);
}

fn main() {
    let h = thread::spawn(|| io_bound(200, "World.\n".to_owned()));
    let k = thread::spawn(|| io_bound(100, "Hello, ".to_owned()));

    let _ = h.join();
    let _ = k.join();
}
