use crate::frame::Frame;
use image::{DynamicImage, GenericImageView};
use nokhwa::{
    Camera,
    pixel_format::RgbFormat,
    utils::{
        CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution,
    },
};

pub struct CameraCapture {
    camera: Camera,
}

impl CameraCapture {
    pub fn new() -> Self {
        let camera = camera_init();
        CameraCapture { camera }
    }

    pub fn capture_frame(&mut self) -> Frame {
        let img = capture_resized_image(&mut self.camera);
        image_to_frame(&img)
    }
}

fn resize_image(image: &DynamicImage) -> DynamicImage {
    // doing all this to preserve scale of image and not distorting it
    // OBS: 80 takes about half the time as compared to 100 and 120
    let (w, h) = image.dimensions();
    let aspect_ratio = w as f32 / h as f32;
    let new_width = 320;
    let new_height = (new_width as f32 / aspect_ratio).round() as u32;
    image.resize(new_width, new_height, image::imageops::FilterType::Nearest)
}

fn camera_init() -> Camera {
    // initialize the camera
    let index = CameraIndex::Index(0);
    let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(CameraFormat::new(
        Resolution::new(640, 480),
        FrameFormat::MJPEG,
        30,
    )));

    let mut camera = Camera::new(index, format).unwrap();
    camera.open_stream().unwrap();
    camera
}

fn capture_image(camera: &mut Camera) -> DynamicImage {
    let frame = camera.frame().unwrap();
    DynamicImage::ImageRgb8(frame.decode_image::<RgbFormat>().unwrap())
}

fn capture_resized_image(camera: &mut Camera) -> DynamicImage {
    let img = capture_image(camera);
    resize_image(&img)
}

// finally we will convert the DynamicImage to Frame
fn image_to_frame(image: &DynamicImage) -> Frame {
    let (width, height) = image.dimensions();
    let rgb = image.to_rgb8();
    let pixels = rgb.into_raw();
    Frame {
        width,
        height,
        pixels,
    }
}
