use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use rand::Rng;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7777").unwrap();
    println!("Starting on port 7777");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let mut rng = rand::thread_rng();
    let n: u16 = rng.gen();
    let contents = n.to_string();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, GET\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}