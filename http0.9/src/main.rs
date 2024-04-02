use std::net::{TcpStream, TcpListener};
use std::io::BufRead;
use std::path::Path;
use std::io::Write;
use std::env;
use std::fs;

struct Http {
    addr: String,
}

impl Http {
    const NOT_FOUND_HTML: &str = "<html><body>Not found<body></html>\r\n";

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
                Ok((stream, _)) => {
                    let mut mutable_stream = stream.try_clone().unwrap();
                    Http::handle_data(&mut mutable_stream) 
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                } 
            }
        }
    }

    fn handle_data(stream: &mut TcpStream) {
        let mut reader = std::io::BufReader::new(stream.try_clone().unwrap());
        let mut buffer = String::new();

        reader.read_line(&mut buffer).unwrap();

        match buffer.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [method, path] if *method == "GET" => {
                let cwd = env::current_dir().unwrap();
                let mut abs_path = format!("{}/public{}", cwd.display(), path);

                while abs_path.contains("../") {
                    abs_path = abs_path.replace("../", "")
                }

                Http::try_send_file(stream, &abs_path)
            }

            _ => {
                Http::error(stream)
            }
        }
    }

    fn try_send_file(stream: &mut TcpStream, filepath: &String) {
        match fs::read_to_string(Path::new(filepath)) {
            Ok(response) => stream.write(response.as_bytes()).unwrap(),
            Err(_) => stream.write(Http::NOT_FOUND_HTML.as_bytes()).unwrap()
        };
    }

    fn error(stream: &mut TcpStream) {
        let cwd = env::current_dir().unwrap();
        let filepath = format!("{}/public/error.html", cwd.display());

        Http::try_send_file(stream, &filepath)
    }
}

fn main() {
    let port = 3333;
    let host = String::from("localhost");

    Http::new(host, port).start();
}
