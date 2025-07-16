use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listner = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection, Established");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("{http_request:#?}");
}
