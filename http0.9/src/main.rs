use std::net::{TcpStream, TcpListener};
use std::fs::read_to_string;
use std::io::BufRead;
use std::path::Path;
use std::io::Write;
use std::env;

struct Http {
    addr: String,
}

impl Http {
    fn new(host: String, port: u16) -> Self {
        Self { addr: format!("{}:{}", host, port) }
    }

    fn start(&self) {
        Self::listen(self.addr.clone());
    }

    fn listen(addr: String) {
        let listener = TcpListener::bind(addr.clone()).unwrap();

        println!("Listening on {}", addr);

        loop {
            match listener.accept() {
                // TODO: Handle the stream in a separate thread to allow multiple concurrent connections
                Ok((stream, addr)) => {
                    let mut mutable_stream = stream.try_clone().unwrap();
                    Http::handle_data(&mut mutable_stream) 
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                } }
        }
    }

    fn handle_data(stream: &mut TcpStream) {
        let mut reader = std::io::BufReader::new(stream.try_clone().unwrap());
        let mut buffer = String::new();

        reader.read_line(&mut buffer).unwrap();

        match buffer.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [method, path] if *method == "GET" => {
                // WARNING: Vulnerable to path traversal attacks
                let cwd = env::current_dir().unwrap();
                let abs_path = format!("{}/public/{}", cwd.display(), path);
                let filepath = Path::new(&abs_path);

                if !filepath.exists() {
                    Http::error(stream);
                    return
                }

                let response = read_to_string(filepath).unwrap();
                stream.write(response.as_bytes()).unwrap();
            }

            _ => {
                Http::error(stream)
            }
        }
    }

    fn error(stream: &mut TcpStream) {
        let cwd = env::current_dir().unwrap();
        let filepath = format!("{}/public/error.html", cwd.display());
        let response = read_to_string(Path::new(&filepath)).unwrap();

        stream.write(response.as_bytes()).unwrap();
    }

}

fn main() {
    let port = 3333;
    let host = String::from("localhost");

    Http::new(host, port).start();
}
