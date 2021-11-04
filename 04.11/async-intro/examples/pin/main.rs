async fn async_write_file(fname: &str, element: String) {
    let _ = tokio::fs::write(fname, element).await;
}

async fn pin_example() -> i32 {
    let array = [1, 2, 3];
    let element = &array[2];
    async_write_file("foo.txt", element.to_string()).await;
    *element
}

#[tokio::main]
async fn main() {
    println!("pin_example {}", pin_example().await)
}
