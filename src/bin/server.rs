use carust::camera::CameraCapture;
use carust::network::send_frame;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Waiting for client...");

    let (mut stream, addr) = listener.accept().unwrap();
    println!("Connected to {addr}");

    // capture a frame then send it
    let mut camera = CameraCapture::new();

    loop {
        let frame = camera.capture_frame();

        send_frame(&mut stream, &frame);
    }
}
