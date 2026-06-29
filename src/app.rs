use minifb::{Key, Window};

use crate::camera::CameraCapture;
use crate::renderer::{frame_to_buffer, init_window, render_frame};

pub struct App {
    camera: CameraCapture,
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl App {
    // this is like a constructor but rust doesnt have them so we implemented ourselves
    pub fn new() -> Self {
        let mut camera = CameraCapture::new();

        // we do this once to get dimensions first, we can avoid this by having hardcoded values
        // but this will be necessary while taking images from files or so..
        let frame = camera.capture_frame();

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
            width,
            height,
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let frame = self.camera.capture_frame();

            render_frame(
                &frame,
                &mut self.window,
                &mut self.buffer,
                self.width,
                self.height,
            );
        }
    }
}
