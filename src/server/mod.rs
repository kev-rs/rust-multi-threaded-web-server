pub mod thread_pool;

use std::{
    fs,
    io::{self, prelude::*, Read, Write}, time::Duration, net, thread,
};

pub struct Request {
    pub method: String,
    pub path: String,
    stream: net::TcpStream,
}

impl Request {
    pub fn new(stream: net::TcpStream) -> Request {
        Request {
            method: String::new(),
            path: String::new(),
            stream,
        }
    }

    pub fn process(&self) -> Request {
        let bufer_read = io::BufReader::new(&self.stream);

        let http_request: Vec<_> = bufer_read
            .lines()
            .map(|res| res.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let req: Vec<_> = http_request[0].split(' ').collect();
        Request { method: String::from(req[0]), path: String::from(req[1]), stream: self.stream.try_clone().unwrap() }
    }
}

type Status = String;
type Body = String;
type Headers = String;

pub struct Response {
    pub status: Status,
    pub body: Body,
    pub headers: Headers,
    request: Request,
}

impl Response {
    pub fn new(request: Request) -> Response {
        Response {
            status: String::new(),
            body: String::new(),
            headers: String::new(),
            request,
        }
    }

    #[allow(unused_assignments)]
    pub fn send(&mut self) -> Result<(), io::Error> {
        let (status_line, body, headers) = match self.request.path.as_str() {
            "/" => {
                let body = match fs::read_to_string("page.html") {
                    Ok(content) => content,
                    Err(err) => create_page(err, "page")?,
                };
                let length = body.len();
                (format!("HTTP/1.1 200 OK"), body, format!("Content-Length: {}", length))
            },
            "/test" => {
                thread::sleep(Duration::from_secs(5));
                let body = match fs::read_to_string("page.html") {
                    Ok(content) => content,
                    Err(err) => create_page(err, "page")?,
                };
                let length = body.len();
                (format!("HTTP/1.1 200 OK"), body, format!("Content-Length: {}", length))
            }
            _ => {
                let body = match fs::read_to_string("404.html") {
                    Ok(content) => content,
                    Err(err) => create_page(err, "404")?,
                };
                let length = body.len();
                (format!("HTTP/1.1 404 NOT FOUND"), body, format!("Content-Length: {}", length))
            }
        };

        let response = format!("{status_line}\r\n{headers}\r\n\r\n{body}");

        self.request.stream.write_all(response.as_bytes())?;

        Ok(())
    }
}

fn create_page(err: io::Error, page_name: &str) -> Result<String, io::Error> {
    match err.kind() {
        io::ErrorKind::NotFound => {
            let mut file = fs::File::create(format!("{}.html", page_name))?;
            let content = match page_name {
                "404" => format!("<!DOCTYPE html>\n<html lang='en'>\n<head>\n<meta charset='UTF-8'>\n<meta name='viewport', content='width=device-width, initial-scale=1.0'>\n<title>Not Found</title>\n</head>\n<body>\n<h1>Page Not Found __404__</h1>\n</body>\n</html>"),
                _ => format!("<!DOCTYPE html>\n<html lang='en'>\n<head>\n<meta charset='UTF-8'>\n<meta name='viewport', content='width=device-width, initial-scale=1.0'>\n<title>Web Server</title>\n</head>\n<body>\n<h1>Web server single-threaded</h1>\n</body>\n</html>"),
            };

            file.write_all(content.as_bytes())?;
            let mut file = fs::File::open("404.html")?;

            let mut body = String::new();
            file.read_to_string(&mut body)?;
            return Ok(body);
        }
        other_err => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{}", other_err),
            ))
        }
    }
}
