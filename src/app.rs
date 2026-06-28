use image::GenericImageView;
use minifb::{Key, Window};
use nokhwa::Camera;

use crate::camera::{camera_init, capture_resized_image};
use crate::renderer::{image_to_buffer, init_window};

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
        let img = capture_resized_image(&mut camera);

        let (width, height) = img.dimensions();

        let window = init_window(width as usize, height as usize);

        let buffer = image_to_buffer(&img);

        // return App
        Self {
            camera,
            window,
            buffer,
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let img = capture_resized_image(&mut self.camera);

            self.buffer = image_to_buffer(&img);

            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();
        }
    }
}
