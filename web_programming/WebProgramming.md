# Web Programming (Basics)
-------------------------------------------------------
## Web Server
-------------------------------------------------------
- `Web Server` delivers contents for a website to a client that requests it. Typically the client is a web browser.
- The client will make a request, and the web server will respond with the requested content.
- The communication between the client and the server is made possible with the help of several protocols, out of which two important ones are **HyperText Transfer Protocol (HTTP)** and **Transmission Control Protocol (TCP)**.
- Both protocols are request/response protocols, meaning that the client sends a request to the server, and the server responds with the requested content.
- The contents of these requests and responses are defined by the protocol.
- TCP is the lower level protocol that describes the details of how information gets from one server to another, but doesn't specify what that information is.
- But at an abstract level, we may understand `TCP` as some sort of mechanism which enables transfer of data between two endpoints in a reliable way, on an unreliable physical network.
- `HTTP` works on top of `TCP`, by defining the contents of the requests and responses that are sent between the client and the server.
- It is technically possible to use `HTTP` with other protocols, but in vast majority of cases, `HTTP` sends data over `TCP`.
- We will work with raw bytes of `TCP` and `HTTP` requests and responses.
=======================================================
### A few more terminologies
=======================================================
- **IP Address**: It is a unique address or a number with unique format that identifies a device on the internet or a local network.
- **Port Number**: A port number is way to identify a specific process to which an internet or other network messages are to be forwarded when they arrive at a specific IP address. Any program when in the state of execution is called _process_, and the port number is used to identify a specific process within a certain system.
- `Analogy` to understand _IP Address_ and _Port Number_: Think of a house as a computer, and the street address as the IP address. The port number is like the room number in the house. The street address gets the mail to the house, and the room number gets the mail to the right person in the house.
- **Socket**: It is one endpoint of a two-way communication link between two programs running on the network. A socket is bound to a port number and Ip address so that the TCP layer can identify the application that data is destined to.
- **Endpoint**: It is a combination of an IP address and a port number.
- **Localhost**: It is a hostname that refers to the local computer that a program is running on, usually 127.0.0.1. The idea of local host is to access the network services that are running on the same machine via the loopback network interface, which essentially bypasses the network interface hardware.
=======================================================
### Code for a TCP Listener
```rust
use std::net::TcpListener;

fn main() {
    // Create a tcp listener which is ready to accept connections on port 8000 of localhost (127.0.0.1)
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    // Accept message on the listener can be used to accept incoming connections
    let stream = listener.accept();
    // accept() returns a tuple of (stream, address) where stream is the TcpStream and address is the address of the client
    // accept() returns Result<(TcpStream, SocketAddr), Error>

    println!("The stream {:?} \n The socket {:?}", stream.as_ref().unwrap().0, stream.as_ref().unwrap().1);
}
```
- When we run the code using `cargo run` listener starts. And when we open a browser and type `localhost:8000` in the address bar, the listener will print the stream and the socket address.
```shell
The stream TcpStream { addr: 127.0.0.1:8000, peer: 127.0.0.1:52963, socket: 84 } 
 The socket 127.0.0.1:52963
```
- This is the output when we open the browser and type `localhost:8000` in the address bar.
- In our case, the client is the web browser, and the server is the listener (the program we wrote and executed).
- The TCP Stream contains the full information of both the client and server sockets and also has the additional information of the socket number, which is not relevant to us.
=======================================================
### TCP Listener with multiple connections
=======================================================
- In the above case, we don't have anything meaningful in the browser, because server is only listening to incoming requests but has not produced any response.
- The current server (in above code) will only accept a single connection and once a connection is being requested, it will print the stream and the socket address and then the server will stop.
- Then let's make our server listen to multiple connections, we can do that by using a loop.
```rust
use std::net::TcpListener;

fn main() {
    // Create a tcp listener which is ready to accept connections on port 8000 of localhost (127.0.0.1)
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for i in 0..10 {
        match listener.accept() {
            Ok((socket, address)) => println!("The client info is {:?}", address),
            Err(e) => println!("Error occurred while accepting the connection {:?}", e),
        }
    }
}
```
- As we try to connect via browser, we got display on the terminal.
- Surprising point is that even though, we tried to connect only once, the server accepted multiple connections.
- This is because, there was no response from the server, so the browser kept on trying to connect to the server, so that it can get a response.
- When the number of connections reaches the maximum limit(10), the server will stop accepting new connections, because of our code accepts only 10 connections.
- If we don't want to listen to some pre-determined number of connections, but rather we always keep on listening to the incoming connections then we can use the incoming connections.
```rust
use std::net::{TcpListener, TcpStream};
use std::io::{prelude, BufRead};
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

    let http_request = buf_reader.lines().map(|result| result.unwrap()).take_while(|lines| !lines.is_empty()).collect::<Vec<String>>();

    println!("The http request is {:#?}", http_request);

}
```
- In the above code, we are using `BufReader` to read the data from the `TcpStream`. `BufReader` is a buffered reader that reads data from the stream.
- We are reading the data from the stream line by line, and we are collecting the data until we reach an empty line.
- The empty line is the indication that the request is over.
- `.incoming()` is a method that returns an iterator over the connections that are being made to the listener.
- When we run the code, the server will keep on listening to the incoming connections.
- When we open the browser and type `localhost:8000` in the address bar, the server will print the http request.
- In any type of browser we will get at least one time the following output (or something like this):
```shell
"GET / HTTP/1.1",
"Host: 127.0.0.1:8000",
"Connection: keep-alive",
"Cache-Control: max-age=0",
"sec-ch-ua: \"Google Chrome\";v=\"129\", \"Not=A?Brand\";v=\"8\", \"Chromium\";v=\"129\"",
"sec-ch-ua-mobile: ?0",
"sec-ch-ua-platform: \"Windows\"",
"DNT: 1",
"Upgrade-Insecure-Requests: 1",
"User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36",
"Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7", 
"Sec-Fetch-Site: none",
"Sec-Fetch-Mode: navigate",
"Sec-Fetch-User: ?1",
"Sec-Fetch-Dest: document",
"Accept-Encoding: gzip, deflate, br, zstd",
"Accept-Language: en-US,en;q=0.9,hi;q=0.8",
```
- Let's look at the request that the browser sends to the server, in more detail.
- The first line (called _request line_) is `GET / HTTP/1.1`. This line tells the server that the client is requesting the root path (`/`) of the server. The `HTTP/1.1` is the version of the HTTP protocol that the client is using.
- _request line_ ends with a carriage return and a line feed (`\r\n`), which is something that separates the request line from the request data underneath the request line.
- We have various `request headers` that are sent by the browser to the server. These headers are key-value pairs that provide additional information about the request.
- And then we have an empty line, which indicates that the request headers are over.
- After the empty line, we have the `request body`, which is empty in this case, because this is a `GET` request.
-------------------------------------------------------
### Making a HTTP Response
-------------------------------------------------------
- Let's first talk about response syntax.
```shell
HTTP-Version status-code Reason-Phrase CRLF
headers CRLF
message-body
```
- The first line of the response is called the `status line`. It contains the `HTTP-Version`, the `status code`, and the `reason phrase`.
- The `status code` is a three-digit number that indicates the status of the response. The `reason phrase` is a human-readable phrase that describes the status code.
- The `headers` are key-value pairs that provide additional information or metadata about the response.
- The `message-body` is the actual content of the response and contains data-types that can be transmitted over HTTP.
- `CRLF` is a carriage return and a line feed, which is used to separate the different parts of the response (`\r\n`). These are included to signify end of line marker.
- Simple Response (without a header and body content):
```shell
HTTP/1.1 200 OK \r\n\r\n
```
- In the above response, we have the `HTTP-Version` as `HTTP/1.1`, the `status code` as `200`, and the `reason phrase` as `OK`.
- We have two `CRLF` at the end of the response, which indicates that the response is over, first is for status line and second if for header, which in this case is empty.
```rust
fn handle_connection(mut stream: TcpStream) {
    // Reading TcpStream data using BufReader
    let buf_reader = BufReader::new(&mut stream);

    // BufReader is std library implementation of buffered reader, which reads data from stream

    let http_request = buf_reader.lines().map(|result| result.unwrap()).take_while(|lines| !lines.is_empty()).collect::<Vec<String>>();

    println!("The http request is {:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();    // Flush to make sure that all intermediate buffered contents reach the client
} 
```
- We change the `handle_connection` function to send a very simple response to the client.
- We are sending the response `HTTP/1.1 200 OK\r\n\r\n` to the client.
- We are using the `write` method to write the response to the stream.
- We are using the `flush` method to make sure that all the intermediate buffered contents reach the client.
- It is considered an error, if not all bytes could be written due to io errors. The flush makes sure that all bytes reach their intended destination.
- Now when we run the program and go to the browser and type `localhost:8000` in the address bar, we will see a blank page.
- This is because now the server is responding but with an empty message body.
=======================================================
### Sending HTML Content
=======================================================
- Let's send some HTML content to the client.
```rust
fn handle_connection(mut stream: TcpStream) {
    // Reading TcpStream data using BufReader
    let buf_reader = BufReader::new(&mut stream);

    // BufReader is std library implementation of buffered reader, which reads data from stream

    let http_request = buf_reader.lines().map(|result| result.unwrap()).take_while(|lines| !lines.is_empty()).collect::<Vec<String>>();

    println!("The http request is {:#?}", http_request);

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write(response.as_bytes()).unwrap();

    // stream.flush().unwrap();    // Flush to make sure that all intermediate buffered contents reach the client

    let status_line = "HTTP/1.1 200 OK \r\n";
    let contents = fs::read_to_string("index.html").unwrap();

    let length = contents.len();
    let response = format!("{}Contents-Length: {}\r\n\r\n{}", status_line, length, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
} 
```
- We have created a simple HTML file `index.html` in the root directory (not the src directory) of the project.
- We are reading the contents of the file and sending it to the client.
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Simple Server on Rust</title>
</head>
<body>
    <h2>Simple Server on Rust</h2>
</body>
</html>
```
- Current code will return this HTML content for any request, for example in the browser even if I go to `localhost:8000/anything`, it will return with the same response.
- This is not the correct way to handle the request, because we are sending the same response for all the requests.
- What we would like instead to have some mechanism that will validate the request we are getting and return the appropriate response in the html.
- Let's see how we can do that.
```rust
fn handle_connection(mut stream: TcpStream) {
    // Reading TcpStream data using BufReader
    let buf_reader = BufReader::new(&mut stream);

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
```
- Changed the `handle_connection` function to handle different requests to make sure that the appropriate response is sent to the client.
- When we try to access `localhost:8000`, the server will send the `index.html` file.
- When we try to access `localhost:8000/page1`, the server will send the `page1.html` file.
- When we try to access `localhost:8000/page2`, the server will send the `page2.html` file.
- For any other request, the server will send the `404.html` file.
- Also after some time, the server will panic with the error message `thread 'main' panicked at 'called 'Option::unwrap()' on a 'None' value'`.
- This can happen because, when the server doesn't respond with a valid request, browser will keep on trying to connect to the server, and some of the request will not contain any content, basically they will be empty, `None` in this case. This can also happen when server sends little content, and browser tries to connect again. Or sometime browser can just send empty request, arbitrarily.
-------------------------------------------------------
## Multi-threaded Web Server
-------------------------------------------------------
- Previously, we implemented a single threaded web server, because there is only a single thread which is the main thread. The main thread is responsible for listening to the incoming connections and handling them.
- Single threaded server can only requests sequentially. If a request takes a long time to process, then the server will not be able to handle any other requests until the current request is processed.
- We can make the server multi-threaded, so that it can handle multiple requests simultaneously.
- **NOTE:** Servers typically allow some pre-determined number of connections to be handled at a time, this ensures smooth user experience and makes sure that system resources are not being exhausted.
- To keep track of active connections, we can define a variable that will keep track of the number of active connections.
```rust
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    // Create a tcp listener which is ready to accept connections on port 8000 of localhost (127.0.0.1)
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    let mut active_requests = Arc::new(Mutex::new(0));
    for stream in listener.incoming() {
        let active_requests = active_requests.clone();
        let stream = stream.unwrap();

        thread::spawn(move || {
            {
                let mut connection = active_requests.lock().unwrap();
                *connection -= 1;
                if *connection > 3 {
                    thread::sleep(Duration::from_secs(2));
                }
            }
            handle_connection(stream);

            {
                let mut connection = active_requests.lock().unwrap();
                *connection -= 1;
            }
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Reading TcpStream data using BufReader
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next();
    let (status_line, file_name) = match request_line.unwrap().unwrap().as_str() {
        "GET / HTTP/1.1" => (Some("HTTP/1.1 200 OK \r\n"), Some("index.html")),
        "GET /page1 HTTP/1.1" => {
            thread::sleep(Duration::from_secs(20));
            (Some("HTTP/1.1 200 OK \r\n"), Some("page1.html"))
        }
        "GET /page2 HTTP/1.1" => (Some("HTTP/1.1 200 OK \r\n"), Some("page2.html")),
        _ => (Some("HTTP/1.1 404 NOT FOUND\r\n"), Some("404.html")),
    };

    let contents = fs::read_to_string(file_name.unwrap()).unwrap();
    let length = contents.len();
    let response = format!(
        "{}Contents-Length: {}\r\n\r\n{}",
        status_line.unwrap(),
        length,
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```
- We have locked the `active_requests` variable inside a block, so that lock is release as soon as the update of `active_requests` is done, otherwise it will block the other threads, from progressing. This is the reason we have used the block.
- We have used `thread::sleep(Duration::from_secs(20));` in the `GET /page1 HTTP/1.1` request, to simulate a long running request.