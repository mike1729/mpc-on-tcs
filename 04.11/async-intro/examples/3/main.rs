async fn io_bound(delay: u64, msg: String) {
    // do some io bound stuff
    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;

    print!("{}", msg)
}

#[tokio::main]
async fn main() {
    let h = tokio::spawn(io_bound(200, "World.\n".to_owned()));
    let k = tokio::spawn(io_bound(100, "Hello, ".to_owned()));

    let _ = tokio::join!(h, k);
}
