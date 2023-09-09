pub mod server;
use crate::server::{thread_pool::ThreadPool, Request, Response};

use std::{
    io::{self},
    net,
};

fn main() -> Result<(), io::Error> {
    let listener: net::TcpListener = net::TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream: net::TcpStream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream).unwrap();
        });
    }

    Ok(())
}

fn handle_connection(stream: net::TcpStream) -> Result<(), io::Error> {
    let request: Request = Request::new(stream);
    let mut response: Response = Response::new(request.process());

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
