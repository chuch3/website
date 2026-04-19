use std::{
    error::Error,
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

mod context;
mod response;
mod thread;
use response::build_get_response;
use thread::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            if let Err(e) = handle_request(stream) {
                println!("Error handling connection {e}")
            }
        });
    }
}

fn handle_request(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf_reader = BufReader::new(&stream);
    let buf = buf_reader.fill_buf().unwrap();

    println!("{}", std::str::from_utf8(buf).unwrap());
    if buf.is_empty() {
        println!("Received empty or invalid request");
        return Ok(());
    }

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let status = req.parse(buf)?;

    let res = match req.method {
        Some("GET") => build_get_response(req, status),
        _ => {}
    };

    /*
    let (status_line, template) = match &http_request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "template/response.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "template/response.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "template/error.html"),
    };

    let contents = fs::read_to_string(template).expect("File should be in directory");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    */
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]

    // Append `-- --no-capture` during tests
    fn test_thread_pool_shutdown() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        let pool = ThreadPool::new(4);

        for stream in listener.incoming().take(2) {
            // After ending iterator, causes compiler to drop threads
            let stream = stream.unwrap();
            pool.execute(|| {
                if let Err(e) = handle_request(stream) {
                    println!("Error handling connection {e}")
                }
            });
        }
        println!("Shutting down");
    }
}
