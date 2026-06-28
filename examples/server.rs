use std::io::Read;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        // receive till the client disconnects
        loop {
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer).unwrap();

            if bytes_read == 0 {
                break;
            }

            let message = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
            println!("Received bytes = {}, '{}'", bytes_read, message);
        }
    }
}
