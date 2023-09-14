use std::{fs, io::Write, net::TcpStream, thread, time::Duration};

pub struct Response {
    status_line: String,
    headers: String,
    body: String,
}

pub struct Request<'a>(pub Method<'a>, pub Path<'a>, pub Protocol<'a>);

type Method<'a> = &'a str;
type Path<'a> = &'a str;
type Protocol<'a> = &'a str;

impl Response {
    pub fn new(Request(method, path, _): Request, mut stream: TcpStream) -> Response {
        let (status_line, headers, body): (String, String, String) = match (method, path) {
            ("GET", "/") => {
                let status_line = "HTTP/1.1 200 OK".to_owned();
                let body = fs::read_to_string("public/home.html").unwrap();
                let headers = format!(
                    "Content-Type: text/html; charset=UTF-8\r\nContent-Length: {}",
                    body.len()
                );

                (status_line, headers, body)
            }
            ("GET", "/test") => {
                thread::sleep(Duration::from_secs(2));
                let status_line = "HTTP/1.1 200 OK".to_owned();
                let body = fs::read_to_string("public/test.html").unwrap();
                let headers = format!(
                    "Content-Type: text/html; charset=UTF-8\r\nContent-Length: {}",
                    body.len()
                );

                (status_line, headers, body)
            },
            ("GET", "/api/user") => {
                let status_line = "HTTP/1.1 200 OK".to_owned();
                let body = fs::read_to_string("public/test.html").unwrap();
                let headers = format!(
                    "Content-Type: text/html; charset=UTF-8\r\nContent-Length: {}",
                    body.len()
                );

                (status_line, headers, body)
            }
            _ => {
                let status_line = "HTTP/1.1 404 NOT FOUND".to_owned();
                let body = "<h1>Not Found</h1>".to_owned();
                let headers = format!(
                    "Content-Type: text/html; charset=UTF-8\r\nContent-Length: {}",
                    body.len()
                );

                (status_line, headers, body)
            }
        };

        let response = format!("{}\r\n{}\r\n\r\n{}", status_line, headers, body);

        stream.write_all(response.as_bytes()).unwrap();

        Response {
            status_line,
            headers,
            body,
        }
    }
}
