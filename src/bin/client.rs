use std::net::TcpStream;

use carust::network::recv_frame;
use carust::renderer::{init_window, render_frame};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    // receive the first frame to get the dimensions
    let frame = recv_frame(&mut stream);
    let width = frame.width as usize;
    let height = frame.height as usize;
    let mut buffer = vec![0u32; width * height];
    let mut window = init_window(width, height);
    render_frame(&frame, &mut window, &mut buffer, width, height);

    loop {
        let frame = recv_frame(&mut stream);

        render_frame(&frame, &mut window, &mut buffer, width, height);
    }
}
