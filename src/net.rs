use crate::http::{HttpRequest, HttpResponse};
use std::io::prelude::*;
use std::net::TcpStream;

pub fn http(_request: HttpRequest) -> std::io::Result<HttpResponse> {
    let mut stream = TcpStream::connect("127.0.0.1:8888")?;

    stream.write(&[1])?;
    let mut buf = [0; 128];
    stream.read(&mut buf)?;

    println!("{:#?}", buf);

    Ok(HttpResponse::new(format!("{:#?}", buf)))
}
