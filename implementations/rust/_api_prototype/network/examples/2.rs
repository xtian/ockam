#![feature(asm)]

use tokio::net::TcpStream;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::io::AsyncReadExt;
use tokio::time::delay_for;
use std::time::Duration;

async fn connect() -> TcpStream {
    println!("Connecting...");
    let mut s = TcpStream::connect(SocketAddr::from_str("127.0.0.1:4050").unwrap()).await.unwrap();
    println!("Connected!");
    s
}

async fn read(s: &mut TcpStream) {
    loop {
        let mut buf = [0; 100];
        println!("Blocking on read...");
        let bytes = s.read(&mut buf).await;
        println!("Read: {}", String::from_utf8(buf.to_vec()).unwrap());
    }
}

async fn say_world() { println!("world"); }

#[tokio::main]
async fn main() {
    // Calling 'connect' doesn't connect.
    let mut tcp = connect();
    println!("not yet connected");
    let mut tcp = tcp.await;

    // creating a 'thread' that blocks in a loop until it reads something on the socket
    tokio::spawn(async move {read(&mut tcp).await});

    let mut sec = 10;
    loop {
        tokio::time::delay_for(Duration::from_millis(1000)).await;
        println!("doing other stuff for {} seconds", sec);
        sec -= 1;
        if 0 == sec {
            break;
        }
    }

}
