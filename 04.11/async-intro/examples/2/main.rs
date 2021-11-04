#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let url = "https://www.tcs.uj.edu.pl";
    let res = client.get(url).send().await;

    print!(
        "Got response: {:?}",
        res.unwrap().headers().get("content-type")
    );
}
