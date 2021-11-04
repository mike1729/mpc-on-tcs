fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let res = rt.block_on(async {
        let client = reqwest::Client::new();
        let url = "https://www.tcs.uj.edu.pl";
        let res = client.get(url).send().await;

        res.unwrap().headers().get("content-type").unwrap().clone()
    });

    print!("Got response: {:?}", res);
}
