use image::{DynamicImage, GenericImageView};
use nokhwa::{Camera, pixel_format::RgbFormat, utils::{CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution}};
use minifb::{Key, Window, WindowOptions};

struct App {
    camera: Camera,
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize
}

impl App {
    // this is like a constructor but rust doesnt have them so we implemented ourselves
    fn new() -> Self {
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
    
    fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let img = capture_resized_image(&mut self.camera);

            self.buffer = image_to_buffer(&img);

            self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
        }
    }
}

fn resize_image(image: &DynamicImage) -> DynamicImage {
    // doing all this to preserve scale of image and not distorting it
    // OBS: 80 takes about half the time as compared to 100 and 120
    let (w,h) = image.dimensions();
    let aspect_ratio = w as f32 / h as f32;
    let new_width = 320;
    let new_height = (new_width as f32 / aspect_ratio).round() as u32;
    image.resize(new_width, new_height, image::imageops::FilterType::Nearest)
}

fn camera_init() -> Camera {
    // initialize the camera
    let index= CameraIndex::Index(0);
    let format = RequestedFormat::new::<RgbFormat>(
        RequestedFormatType::Exact(CameraFormat::new(
            Resolution::new(640,480),
            FrameFormat::MJPEG,
            30
        ))
    );

    let mut camera = Camera::new(index, format).unwrap();
    camera.open_stream().unwrap();
    camera
}

fn capture_image(camera: &mut Camera) -> DynamicImage {
    let frame = camera.frame().unwrap();
    DynamicImage::ImageRgb8(
        frame.decode_image::<RgbFormat>().unwrap()
    )
}

fn capture_resized_image(camera: &mut Camera) -> DynamicImage {
    let img = capture_image(camera);
    resize_image(&img)
}

// opening a window which shows the captured frame in it
// flow: capture image -> convert to rgba -> convert to u32 buffer -> use buffer update 
fn init_window(width: usize, height: usize) -> Window {
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

fn image_to_buffer(img: &DynamicImage) -> Vec<u32> {

    // converting to rgba (0RGB, 32 bytes -> first byte is unused) [because minifb needs in u32 buffer]
    let rgb_image = img.to_rgba8();

    // rgba: each color is 8 bits
    let buffer: Vec<u32> = rgb_image
    .pixels()
    .map(|pixel| {
        let [r, g, b, _] = pixel.0;
        ((r as u32) << 16)
            | ((g as u32) << 8)
            | (b as u32)
    })
    .collect();

    buffer
}

fn main() {
    let mut app = App::new();
    app.run();
}
