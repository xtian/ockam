use std::net::{TcpListener, SocketAddr};
use std::str::FromStr;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind(SocketAddr::from_str("127.0.0.1:4050").unwrap()).unwrap();
    let (mut s, _) = listener.accept().unwrap();
    s.set_nodelay(true);
    println!("Connected");
    let mut buf: String = "".into();
    while buf != "q" {
        println!("type something: ");
        if std::io::stdin().read_line(&mut buf).is_ok() {
            s.write(buf.as_ref());
        }
    }
}
