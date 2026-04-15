use std::io::{stdout, Write};
use image::{DynamicImage, GenericImageView};
use nokhwa::{Camera, pixel_format::RgbFormat, utils::{CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution}};

fn resize_image(image: &DynamicImage) -> DynamicImage {
    // doing all this to preserve scale of image and not distorting it
    // OBS: 80 takes about half the time as compared to 100 and 120
    let (w,h) = image.dimensions();
    let aspect_ratio = w as f32 / h as f32;
    let new_width = 80;
    let new_height = (new_width as f32 / aspect_ratio / 2.0).round() as u32;
    image.resize(new_width, new_height, image::imageops::FilterType::Nearest)
}

fn convert_to_grayscale(image: &DynamicImage) -> DynamicImage {
    image.grayscale()
}

fn convert_to_ascii(grayscale_image: &DynamicImage) -> String {
    // mapping character to brightness
    // TODO: use a better ramp and use gamma
    let ramp: Vec<char> = " .:-=+*#%@".chars().collect();
    let ramp_len = ramp.len() as u32;
    let mut output = String::new();
    let (width, height) = grayscale_image.dimensions();
    for row in 0..height {
        for col in 0..width {
            let pixel = grayscale_image.get_pixel(col, row);
            let brightness = pixel[0];
            let idx = brightness as u32 * (ramp_len - 1) / 255;
            output.push(ramp[idx as usize]);
        }
        output.push('\n');
    }
    output
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


fn main() {
    // initialize the camera
    let mut camera = camera_init();

    print!("\x1B[2J");

    // simulating video in terminal
    loop {
        // \x1B[1;1H moves cursor to the top left
        print!("\x1B[1;1H");

        let img = capture_image(&mut camera);

        // resizing image
        let resized_image = resize_image(&img);

        // converting to grayscale
        let grayscale_image = convert_to_grayscale(&resized_image);

        // convert to ascii
        let output = convert_to_ascii(&grayscale_image);
        print!("{}",output);
        stdout().flush().unwrap();
        
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
