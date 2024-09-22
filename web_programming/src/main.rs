//----------------------------------------------
//      Web Programming Basics
//----------------------------------------------

use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // Create a tcp listener which is ready to accept connections on port 8000 of localhost (127.0.0.1)
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Reading TcpStream data using BufReader
    let buf_reader = BufReader::new(&mut stream);

    // BufReader is std library implementation of buffered reader, which reads data from stream

    // let http_request = buf_reader.lines().map(|result| result.unwrap()).take_while(|lines| !lines.is_empty()).collect::<Vec<String>>();

    // println!("The http request is {:#?}", http_request);

    // // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // // stream.write(response.as_bytes()).unwrap();

    // // stream.flush().unwrap();    // Flush to make sure that all intermediate buffered contents reach the client

    // let status_line = "HTTP/1.1 200 OK \r\n";
    // let contents = fs::read_to_string("index.html").unwrap();

    // let length = contents.len();
    // let response = format!("{}Contents-Length: {}\r\n\r\n{}", status_line, length, contents);

    // stream.write_all(response.as_bytes()).unwrap();
    // stream.flush().unwrap();

    let request_line = buf_reader.lines().next();
    let (status_line, file_name) = match request_line.unwrap().unwrap().as_str() {
        "GET / HTTP/1.1" => (Some("HTTP/1.1 200 OK \r\n"), Some("index.html")),
        "GET /page1 HTTP/1.1" => (Some("HTTP/1.1 200 OK \r\n"), Some("page1.html")),
        "GET /page2 HTTP/1.1" => (Some("HTTP/1.1 200 OK \r\n"), Some("page2.html")),
        _ => (Some("HTTP/1.1 404 NOT FOUND\r\n"), Some("404.html")),
    };

    let contents = fs::read_to_string(file_name.unwrap()).unwrap();
    let length = contents.len();
    let response = format!("{}Contents-Length: {}\r\n\r\n{}", status_line.unwrap(), length, contents);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
} 