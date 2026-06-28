use std::io::Write;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    for _ in 0..5 {
        stream.write_all(b"hello\n").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        stream.flush().unwrap();
    }
}
