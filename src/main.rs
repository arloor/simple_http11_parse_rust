use bytes::{BufMut, BytesMut};
use httparse::{Request, Status};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = BytesMut::with_capacity(4096); // 初始化BytesMut缓冲区
    let mut buf = [0; 4096]; // 临时缓冲区
    loop {
        // 从流中读取数据
        match stream.read(&mut buf) {
            Ok(0) => {
                // 客户端关闭连接
                println!("Client closed the connection");
                return;
            }
            Ok(n) => {
                // 将读取到的数据追加到动态缓冲区
                buffer.put(&buf[0..n]);

                // 尝试解析累积的数据
                let mut headers = [httparse::EMPTY_HEADER; 16];
                let mut request = Request::new(&mut headers);
                match request.parse(&buffer) {
                    Ok(Status::Complete(_)) => {
                        // 成功接收到完整的HTTP请求
                        println!(
                            "Received a complete HTTP request; {} {}",
                            request.method.unwrap_or(""),
                            request.path.unwrap_or("")
                        );

                        // 处理请求
                        let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
                        stream.write_all(response.as_bytes()).unwrap();
                        return;
                    }
                    Ok(Status::Partial) => {
                        // 请求不完整，继续读取更多数据
                        println!("Received partial HTTP request, waiting for more data...");
                        // 不清空缓冲区，继续累积数据
                    }
                    Err(e) => {
                        // 解析错误
                        eprintln!("Failed to parse HTTP request: {:?}", e);
                        return;
                    }
                }
            }
            Err(e) => {
                // 读取数据时出错
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
