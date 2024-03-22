use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let tpool_size = 4;

    let pool = match ThreadPool::build(tpool_size) {
        Ok(pool) => pool,
        Err(error) => panic!("{error}"),
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    if let Some(Ok(request_line)) = BufReader::new(&mut stream).lines().next() {
        let (status_line, filename) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "bad.html"),
        };

        let filename = format!("src/templates/{}", filename);

        if let Ok(contents) = fs::read_to_string(filename) {
            let length = contents.len();
            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            if let Err(err) = stream.write_all(response.as_bytes()) {
                eprintln!("Error writing response: {:?}", err);
            }
        } else {
            eprintln!("Error reading file");
        }
    } else {
        eprintln!("Error reading request line");
    }
}