use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1729").await.unwrap();

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Connected to {:?}", addr);

        tokio::spawn(process(socket));
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        if let Frame::Array(v) = frame {
            println!("GOT: {:?}", v[1].to_string());

            connection.write_frame(&v[1]).await.unwrap();
        }
    }
}
