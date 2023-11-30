#![allow(dead_code, unused)]

use rand::Rng;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Listening on :8080");

    loop {
        let (socket, _) = listener.accept().await?;

        // use make_requests.sh
        // & PICK ONE:

        // parallel_handler(socket).await;
        non_parallel_or_concurrent_handler(socket).await;
    }
}

async fn parallel_handler(mut socket: TcpStream) {
    tokio::spawn(async move {
        let id = rand::thread_rng().gen_range(10000..99999);
        println!("->> [{id}] start");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        socket.write_all(&create_response(id)).await.unwrap();
        println!("->> [{id}] end");
    });
}

async fn non_parallel_or_concurrent_handler(mut socket: TcpStream) {
    let id = rand::thread_rng().gen_range(10000..99999);
    println!("->> [{id}] start");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    socket.write_all(&create_response(id)).await.unwrap();
    println!("->> [{id}] end");
}

fn create_response(id: usize) -> Vec<u8> {
    let content = format!(
        r#"{{"id":"{}","data":"Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."}}"#,
        id
    );

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length:{}\r\n\r\n{content}",
        content.len()
    )
    .as_bytes()
    .to_vec()
}
