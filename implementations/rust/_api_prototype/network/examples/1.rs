use tokio::net::TcpStream;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::io::AsyncReadExt;

async fn connect() -> TcpStream {
    println!("Connecting...");
    let mut s = TcpStream::connect(SocketAddr::from_str("127.0.0.1:4050").unwrap()).await.unwrap();
    println!("Connected!");
    s
}

async fn read(s: &mut TcpStream) -> [u8;100] {
    let mut buf = [0;100];
    println!("reading...");
    let bytes = s.read(&mut buf).await;
    println!("read: {}", String::from_utf8(buf.to_vec()).unwrap());
    buf
}

async fn say_world() { println!("world"); }

#[tokio::main]
async fn main() {
    // Calling 'connect' doesn't connect.
    let mut tcp = connect();
    println!("not yet connected");
    let mut tcp = tcp.await;

    // Calling 'read()' doesn't execute the read
    let b = read(&mut tcp);
    println!("not yet read");

    // Calling `.await` on `b` starts 'read'.
    let buf = b.await;
    println!("main got: {}", String::from_utf8(buf.to_vec()).unwrap());
    println!("done awaiting");
}
