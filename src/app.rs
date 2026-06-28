use minifb::{Key, Window};
use nokhwa::Camera;

use crate::camera::{camera_init, capture_frame};
use crate::renderer::{frame_to_buffer, init_window};

pub struct App {
    camera: Camera,
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl App {
    // this is like a constructor but rust doesnt have them so we implemented ourselves
    pub fn new() -> Self {
        let mut camera = camera_init();

        // we do this once to get dimensions first, we can avoid this by having hardcoded values
        // but this will be necessary while taking images from files or so..
        let frame = capture_frame(&mut camera);

        let width = frame.width as usize;
        let height = frame.height as usize;

        let window = init_window(width, height);

        let mut buffer = vec![0u32; height * width];
        frame_to_buffer(&frame, &mut buffer);

        // return App
        Self {
            camera,
            window,
            buffer,
            width: width,
            height: height,
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let frame = capture_frame(&mut self.camera);

            frame_to_buffer(&frame, &mut self.buffer);

            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();
        }
    }
}
