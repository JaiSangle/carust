use crate::frame::Frame;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

// protocol is: [u32 width] [u32 height] [pixel bytes]
// from first 8 bytes (width and height) we can find how much remaining bytes we need to check

// using big endian to send
pub fn send_frame(stream: &mut TcpStream, frame: &Frame) {
    stream.write_all(&u32_to_bytes(frame.width)).unwrap();
    stream.write_all(&u32_to_bytes(frame.height)).unwrap();
    stream.write_all(&frame.pixels).unwrap();
}

// til: there are two ways machines store numbers: {little endian, big endian}
fn bytes_to_u32(buf: &[u8; 4]) -> u32 {
    ((buf[0] as u32) << 24) | ((buf[1] as u32) << 16) | ((buf[2] as u32) << 8) | (buf[3] as u32)
}

// we have to convert u32 into 4 bytes: {0, 1, 2, 3}
fn u32_to_bytes(number: u32) -> [u8; 4] {
    [
        (number >> 24) as u8,
        (number >> 16) as u8,
        (number >> 8) as u8,
        number as u8,
    ]
}

// using big endian to receive
pub fn recv_frame(stream: &mut TcpStream) -> Frame {
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).unwrap();
    let width = bytes_to_u32(&buf);
    stream.read_exact(&mut buf).unwrap();
    let height = bytes_to_u32(&buf);

    let mut pixel_data = vec![0u8; (height * width * 3) as usize];
    stream.read_exact(&mut pixel_data).unwrap();

    Frame {
        width,
        height,
        pixels: pixel_data,
    }
}
