use carust::frame::Frame;
use carust::network::send_frame;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Waiting for client...");

    let (mut stream, addr) = listener.accept().unwrap();
    println!("Connected to {addr}");

    loop {
        let frame = Frame {
            width: 2,
            height: 2,
            pixels: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255],
        };

        send_frame(&mut stream, &frame);
    }
}
