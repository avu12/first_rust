use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

extern crate postgres;
use postgres::{Connection, TlsMode};

struct Data {
    id: i32,
    text: String
}

fn main() {
    println!("Start!");
    let conn = Connection::connect("postgres://postgres:postgres@192.168.1.10:5432", TlsMode::None)
        .unwrap();

    for row in &conn.query("SELECT * FROM public.testtable", &[]).unwrap() {
        let data = Data {
            text : row.get(0),
            id : row.get(1)
        };
        println!("Data:  {} {}",data.text,data.id);
    }
    println!("End!");
    println!("Testing docker push&pull!");
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }


}

use std::fs;
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}