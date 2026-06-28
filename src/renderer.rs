use crate::frame::Frame;
use minifb::{Window, WindowOptions};

// opening a window which shows the captured frame in it
// flow: capture image -> convert to rgba -> convert to u32 buffer -> use buffer update
pub fn init_window(width: usize, height: usize) -> Window {
    Window::new(
        "Test",
        width,
        height,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open window")
}

pub fn frame_to_buffer(frame: &Frame, buffer: &mut Vec<u32>) {
    // converting to rgba (0RGB, 32 bytes -> first byte is unused) [because minifb needs in u32 buffer]
    // a buffer need u32 which is 0rgb
    // we have r g b -> make a mask that represents 0rgb
    // r should be shifted left by 16
    // g should be shifted left by 8
    // b should be shifted by 0
    let mut i = 0;
    while i < frame.pixels.len() {
        let r = frame.pixels[i] as u32;
        let g = frame.pixels[i + 1] as u32;
        let b = frame.pixels[i + 2] as u32;
        let mask: u32 = (r << 16) | (g << 8) | b;
        buffer[i / 3] = mask;
        i += 3;
    }
}
