use crate::url::ParsedUrl;
use dns_lookup::lookup_host;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;
use std::string::String;
use std::vec::Vec;

/*
struct Header {
    key: String,
    value: String,
}

impl Header {
    fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
*/

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, url: &ParsedUrl) -> std::io::Result<HttpResponse> {
        let ips: Vec<std::net::IpAddr> = lookup_host(&url.host)?;

        let mut stream = TcpStream::connect((ips[0], url.port))?;

        let mut request = String::from("GET /");
        request.push_str(&url.path);
        request.push_str(" HTTP/1.1\n");

        // headers
        request.push_str("Host: ");
        request.push_str(&url.host);
        request.push('\n');
        request.push_str("Accept: */*\n");

        request.push('\n');

        println!("request: {:?}", request);

        stream.write(request.as_bytes())?;

        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;

        Ok(HttpResponse::new(buf))
    }

    // TODO: support correctly
    pub fn post(&self, url: &ParsedUrl, body: String) -> std::io::Result<HttpResponse> {
        let ips: Vec<std::net::IpAddr> = lookup_host(&url.host)?;

        let mut stream = TcpStream::connect((ips[0], url.port))?;

        let mut request = String::from("POST ");
        request.push_str(&url.path);
        request.push_str(" HTTP/1.1\n");

        /*
        // headers
        for h in &url.headers {
            request.push_str(&h.key);
            request.push_str(": ");
            request.push_str(&h.value);
            request.push('\n');
        }
        */

        request.push('\n');

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
