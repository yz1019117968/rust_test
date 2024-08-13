// use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
// use std::fs;

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         println!("connection established!");
//         // handle_connection(stream);
//     }
// }

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 512];

//     stream.read(&mut buffer).unwrap();

//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
//     let contents = fs::read_to_string("hello.html").unwrap();
//     let response = format!("HTTP/1.1 200 OK\r\n\
//          Content-Length: {}\r\n\
//          Content-Type: text/html\r\n\
//          \r\n\
//          {}",
//          contents.len(),
//          contents);

//     stream.write(response.as_bytes()).unwrap();
//     // stream.flush().unwrap();
// }
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::fs;
fn main() {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address).expect("Failed to bind address");
    println!("Server is listening on http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // Read the request
    let _ = stream.read(&mut buffer).expect("Failed to read request");
    
    // Convert the request to a string
    let request = str::from_utf8(&buffer).expect("Failed to convert buffer to string");
    
    // Log the request to the console
    println!("Received request:\n{}", request);

    // GET / HTTP/1.1\r\n 访问主目录。 GET /wewe HTTP/1.1\r\n 访问/wewe目录
    let get = b"GET / HTTP/1.1\r\n";
    let http_response;
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();
        http_response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Length: {}\r\n\
             Content-Type: text/html\r\n\
             \r\n\
             {}",
             contents.len(),
             contents
        );
    }
    else {
        let contents = fs::read_to_string("404.html").unwrap();
        http_response = format!(
            "HTTP/1.1 404 NOT FOUND\r\n\
             Content-Length: {}\r\n\
             Content-Type: text/html\r\n\
             \r\n\
             {}",
             contents.len(),
             contents
        );
    };
    // // Prepare the HTML response
    // let contents = fs::read_to_string("hello.html").unwrap();
    // // Create the complete HTTP response
    // let http_response = format!(
    //     "HTTP/1.1 200 OK\r\n\
    //      Content-Length: {}\r\n\
    //      Content-Type: text/html\r\n\
    //      \r\n\
    //      {}",
    //      contents.len(),
    //      contents
    // );

    // Send the response back to the client
    stream.write_all(http_response.as_bytes()).expect("Failed to write response");
}
