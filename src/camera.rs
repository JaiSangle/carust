use image::{DynamicImage, GenericImageView};
use nokhwa::{
    Camera,
    pixel_format::RgbFormat,
    utils::{
        CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution,
    },
};

pub fn resize_image(image: &DynamicImage) -> DynamicImage {
    // doing all this to preserve scale of image and not distorting it
    // OBS: 80 takes about half the time as compared to 100 and 120
    let (w, h) = image.dimensions();
    let aspect_ratio = w as f32 / h as f32;
    let new_width = 320;
    let new_height = (new_width as f32 / aspect_ratio).round() as u32;
    image.resize(new_width, new_height, image::imageops::FilterType::Nearest)
}

pub fn camera_init() -> Camera {
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

pub fn capture_image(camera: &mut Camera) -> DynamicImage {
    let frame = camera.frame().unwrap();
    DynamicImage::ImageRgb8(frame.decode_image::<RgbFormat>().unwrap())
}

pub fn capture_resized_image(camera: &mut Camera) -> DynamicImage {
    let img = capture_image(camera);
    resize_image(&img)
}
