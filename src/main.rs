pub mod server;
use crate::server::{ Request, Response };

use std::{
    io::{self, prelude::*},
    net,
};

fn main() -> Result<(), io::Error> {
    let listener: net::TcpListener = net::TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream: net::TcpStream = stream.unwrap();

        handle_connection(stream)?;
    }

    Ok(())
}

fn handle_connection(mut stream: net::TcpStream) -> Result<(), io::Error> {
    let buf_reader = io::BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = &http_request[0];
    let request_line: Vec<_> = request_line.split(' ').collect();

    let request: Request<'_> = Request::new(&request_line[0], &request_line[1]);

    let mut response: Response<'_> = Response::new(request, stream);

    response.send()?;

    Ok(())

    /* This is for non-UTF-8 streams */
    //let mut buffer: [u8; 1024] = [0; 1024];

    //while let Ok(bytes_read) = stream.read(&mut buffer) {
    //    if bytes_read == 0 {
    //        break;
    //    }

    //    let data: &[u8] = &buffer[..bytes_read];

    //    if let Ok(text) = str::from_utf8(data) {
    //        // Successfully interpreted as UTF-8 text.
    //        println!("Received text: {}", text);
    //    } else {
    //        // Not valid UTF-8, print as bytes.
    //        println!("Received bytes: {:?}", data);
    //    }
    //}
}
