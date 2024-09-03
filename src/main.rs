use httparse::{Request, Status};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 4096];
    let mut request = Request::new(&mut []);

    loop {
        // Read data from the stream
        match stream.read(&mut buf) {
            Ok(0) => {
                // Connection closed by the client
                println!("Client closed the connection");
                return;
            }
            Ok(n) => {
                // Parsing the HTTP request
                let mut headers = [httparse::EMPTY_HEADER; 16];
                request = Request::new(&mut headers);

                match request.parse(&buf[..n]) {
                    Ok(Status::Complete(_)) => {
                        // Successfully received a complete HTTP request
                        println!("Received a complete HTTP request");

                        // You can now handle the request
                        let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
                        stream.write_all(response.as_bytes()).unwrap();
                        return;
                    }
                    Ok(Status::Partial) => {
                        // Partial request, need more data
                        println!("Received partial HTTP request, waiting for more data...");
                        // Continue the loop to read more data
                    }
                    Err(e) => {
                        // Parsing error
                        eprintln!("Failed to parse HTTP request: {:?}", e);
                        return;
                    }
                }
            }
            Err(e) => {
                // Error reading from the stream
                eprintln!("Failed to read from the stream: {:?}", e);
                return;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {:?}", e);
            }
        }
    }
}
