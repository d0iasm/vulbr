use crate::http::{HttpRequest, HttpResponse};
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;

pub fn http(request: HttpRequest) -> std::io::Result<HttpResponse> {
    let mut stream = TcpStream::connect("127.0.0.1:8888")?;

    stream.write(&request.string().as_bytes())?;

    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;

    Ok(HttpResponse::new(buf))
}
