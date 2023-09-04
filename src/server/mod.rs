use std::{
    fs,
    io::{self, Read, Write},
};

pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(method: &'a str, path: &'a str) -> Request<'a> {
        Request { method, path }
    }
}

type Status = String;
type Body = String;
type Headers = String;

pub struct Response<'a> {
    pub status: Status,
    pub body: Body,
    pub headers: Headers,
    request: Request<'a>,
    stream: std::net::TcpStream,
}

impl<'a> Response<'a> {
    pub fn new(request: Request<'a>, stream: std::net::TcpStream) -> Response {
        Response {
            status: String::new(),
            body: String::new(),
            headers: String::new(),
            request,
            stream,
        }
    }

    #[allow(unused_assignments)]
    pub fn send(&mut self) -> Result<(), io::Error> {
        let (status_line, body, headers) = match self.request.path {
            "/" => {
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

        self.stream.write_all(response.as_bytes())?;

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
