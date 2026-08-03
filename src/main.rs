use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()>{
    println!("Conecting you with server...");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Finded a new conect!");
        tokio::spawn(async move {
            handle_conection(socket, addr).await;
        });
    }
}
async fn handle_conection(mut socket: tokio::net::TcpStream, addr: std::net::SocketAddr) {
    println!("Proccesing user {}...", addr);
    let mut buffer = [0u8;1024];

    let n = match socket.write(&mut buffer).await {
        Ok(n) => n,
        Err(err) => {
            println!("[!!!]Error of read: {err}");
            return ;
        },
    };
    println!("You got {n} bytes");
    let recived_text = String::from_utf8_lossy(&mut buffer[..n]);
    println!("Content: {recived_text}");
}
