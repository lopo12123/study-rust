use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use study_rust::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    // 请求
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

    // 响应
    // !注意: 浏览器安全策略, 响应头需要有 Content-Length 字段才能被正确接收
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body

    if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        let contents = fs::read_to_string("./pages/index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let contents = fs::read_to_string("./pages/404.html").unwrap();
        let response = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}