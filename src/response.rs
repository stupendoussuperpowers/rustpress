use std::{collections::HashMap, io::Write, net::TcpStream};

#[derive(Debug)]
pub struct Response<'a> {
    pub content: String,
    pub status: i32,
    pub stream: &'a mut TcpStream,
    pub headers: HashMap<String, String>,
}

impl Response<'_> {
    pub fn send(&mut self) {
        self.stream
            .write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    self.content.len(),
                    self.content
                )
                .as_bytes(),
            )
            .unwrap();
    }
}
