use crate::url::ParsedUrl;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;
use std::string::String;
use std::vec::Vec;

#[derive(Debug)]
struct Header {
    key: String,
    value: String,
}

impl Header {
    fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    host: String,
    path: String,
    version: String,
    headers: Vec<Header>,
    body: String,
}

impl HttpRequest {
    // TODO: remove `method` and add get()/post()/put() etc. functions instead.
    pub fn new(url: &ParsedUrl) -> Self {
        let mut req = Self {
            host: url.host.clone(),
            path: String::from(&url.path),
            version: String::from("HTTP/1.1"),
            headers: Vec::new(),
            body: String::from("sending a request"),
        };

        req.add_header(String::from("Host"), String::from(&url.host));

        req
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.push(Header::new(key, value));
    }

    pub fn get(&self) -> std::io::Result<HttpResponse> {
        let mut stream = TcpStream::connect(&self.host)?;

        let mut request = String::from("GET ");
        request.push_str(&self.path);
        request.push(' ');
        request.push_str(&self.version);
        request.push('\n');

        // headers
        for h in &self.headers {
            request.push_str(&h.key);
            request.push_str(": ");
            request.push_str(&h.value);
            request.push('\n');
        }
        request.push('\n');

        // body
        request.push_str(&self.body);

        stream.write(request.as_bytes())?;

        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;

        Ok(HttpResponse::new(buf))
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
    // TODO: replace String with Vec<Header>.
    headers: String,
    body: String,
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Self {
        let preprocessed_response = raw_response.replace("\n\r", "\n");

        let (status_line, remaining) = match preprocessed_response.split_once("\n") {
            Some((s, r)) => (s, r),
            None => panic!("http response doesn't have a new line"),
        };

        let (headers, body) = match remaining.split_once("\n\n") {
            Some((h, b)) => (h, b),
            None => ("", remaining),
        };

        let statuses: Vec<&str> = status_line.split(" ").collect();

        Self {
            version: statuses[0].to_string(),
            status_code: match statuses[1].parse() {
                Ok(s) => s,
                Err(_) => 404,
            },
            reason: statuses[2].to_string(),
            headers: headers.to_string(),
            body: body.to_string(),
        }
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }
}
