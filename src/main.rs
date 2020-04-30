use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connected");

        handle_con(stream);
    }
}

fn handle_con(mut stream: TcpStream){
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    //println!("request {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get){

    let cont = fs::read_to_string("sample_html.html").unwrap();

    let resp = format!("HTTP/1.1 200 OK\r\n\r\n{}",cont);

    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
    } else {

    }
}