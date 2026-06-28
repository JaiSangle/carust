use image::DynamicImage;
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

pub fn image_to_buffer(img: &DynamicImage) -> Vec<u32> {
    // converting to rgba (0RGB, 32 bytes -> first byte is unused) [because minifb needs in u32 buffer]
    let rgb_image = img.to_rgba8();

    // rgba: each color is 8 bits
    let buffer: Vec<u32> = rgb_image
        .pixels()
        .map(|pixel| {
            let [r, g, b, _] = pixel.0;
            ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        })
        .collect();

    buffer
}
