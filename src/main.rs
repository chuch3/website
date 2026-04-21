use dotenv_codegen::dotenv;
use std::{
    error::Error,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

mod context;
mod response;
mod thread;
use context::Context;
use response::build_get_response;
use thread::ThreadPool;

fn main() {
    let listener = TcpListener::bind(dotenv!("DOMAIN_NAME")).unwrap();
    let pool = ThreadPool::new(4);

    for tcp_stream in listener.incoming() {
        let tcp_stream = tcp_stream.unwrap();
        pool.execute(move |ctx| {
            if let Err(e) = handle_request(tcp_stream, ctx) {
                println!("Error handling connection {e}")
            }
        });
    }
}

fn handle_request(mut stream: TcpStream, ctx: &Context) -> Result<(), Box<dyn Error>> {
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
        Some("GET") => build_get_response(ctx, req, status)?,
        Some(_) => todo!(), // Error page here
        None => response::cont()?,
    };

    stream.write_all(&http_bytes::response_header_to_vec(&res))?;
    stream.write_all(res.body())?;

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
            pool.execute(move |ctx| {
                if let Err(e) = handle_request(stream, ctx) {
                    println!("Error handling connection {e}")
                }
            });
        }
        println!("Shutting down");
    }
}
