use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let conn = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("3333");
    for stream in conn.incoming() {
        let mut _str = stream.unwrap();
        println!("connection established");
        let mut buf = [0; 1024];
        _str.read(&mut buf).unwrap();
        _str.write(&mut buf).unwrap();
    }
}
