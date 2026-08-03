use tokio::net::TcpListener;

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
async fn handle_conection(socket: tokio::net::TcpStream, addr: std::net::SocketAddr) {
    println!("Proccesing user {}...", addr);
}
