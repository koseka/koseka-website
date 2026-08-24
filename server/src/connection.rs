use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

pub(crate) fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();
    let (status_line, filename, content_type) = match request_line.split_whitespace().nth(1) {
        Some("/") => ("HTTP/1.1 200 OK", "index.html", "text/html; charset=utf-8"),
        Some("/index.html") => ("HTTP/1.1 200 OK", "index.html", "text/html; charset=utf-8"),
        Some("/wip.gif") => ("HTTP/1.1 200 OK", "wip.gif", "image/gif"),
        Some("/favicon.ico") => ("HTTP/1.1 200 OK", "favicon.ico", "image/x-icon"),
        Some(_) => ("HTTP/1.1 404 NOT FOUND", "404.html", "text/html; charset=utf-8"),
        None => ("HTTP/1.1 404 NOT FOUND", "404.html", "text/html; charset=utf-8"),
    };
    let mut path = String::from("../assets/");
    path.push_str(filename);
    let head: String;
    let contents = fs::read(&path).unwrap_or_else(|_| b"404 Not Found".to_vec());

    if Path::new(path.as_str()).exists() {
        head = format!("{}Content-Length: {}\r\nContent-Type: {}\r\n\r\n", status_line, contents.len(), content_type);
    } else {
        let msg = "404 Not Found";
        head = format!("{}Content-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}", status_line, msg.len(), msg);
    }

    stream.write_all(head.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
    stream.flush().unwrap();
}
