mod server;
use server::{response::*, thread_pool::ThreadPool};

use std::{
    io::{self, Read},
    net,
};

fn main() -> Result<(), io::Error> {
    let listener = net::TcpListener::bind("127.0.0.1:7171")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_client(stream).unwrap();
        });
    }

    println!("Server shutting down...");

    Ok(())
}

fn handle_client(mut stream: net::TcpStream) -> Result<(), io::Error> {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer)?;

    let http_request = String::from_utf8_lossy(&buffer[..]);
    let req = parse_request(&http_request);

    let _res = Response::new(req, stream);

    Ok(())
}

type Method<'a> = &'a str;
type Path<'a> = &'a str;
type Protocol<'a> = &'a str;

fn parse_request<'a>(request: &'a str) -> Request<'a> {
    let mut request = request.lines();
    let request_line: Vec<_> = request.next().unwrap().split(' ').collect();

    let method: Method = request_line[0];
    let path: Path = request_line[1];
    let protocol: Protocol = request_line[2];

    Request(method, path, protocol)
}
