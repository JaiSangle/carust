use std::net::TcpStream;

use carust::frame::Frame;
use carust::network::recv_frame;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    loop {
        let frame = recv_frame(&mut stream);

        assert_eq!(
            frame,
            Frame {
                width: 2,
                height: 2,
                pixels: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255]
            }
        );
    }
}
