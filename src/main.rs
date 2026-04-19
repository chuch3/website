use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

mod thread;
use thread::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request = buf_reader.lines().next().unwrap().unwrap();

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
            pool.execute(|| handle_connection(stream));
        }
        println!("Shutting down");
    }
}
