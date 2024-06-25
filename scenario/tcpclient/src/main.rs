use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3333").unwrap();
    stream.write("hello".as_bytes()).unwrap();
    let mut buf = [0; 5];
    stream.read(&mut buf).unwrap();
    println!(
        "got response from servr:{:?}",
        str::from_utf8(&buf).unwrap()
    );
}
