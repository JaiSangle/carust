use image::{DynamicImage, ImageReader, GenericImageView};

fn resize_image(image: &DynamicImage) -> DynamicImage {
    let res = image.resize(100, 50, image::imageops::FilterType::Nearest);
    res
}

fn convert_to_grayscale(image: &DynamicImage) -> DynamicImage {
    let gray = image.grayscale();
    gray
}

fn convert_to_ascii(grayscale_image: &DynamicImage) -> String {
    // mapping character to brightness
    let ramp: Vec<char> = " .^*+=#&$@".chars().collect();
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


fn main() {
    // load the image using image reader
    let img = ImageReader::open("images/red-moon.png").unwrap().decode().unwrap();
    
    // resizing image
    let resized_image = resize_image(&img);

    // converting to grayscale
    let grayscale_image = convert_to_grayscale(&resized_image);

    // simulating video in terminal
    loop {
        // \x1B[2J clears screen, \x1B[1;1H moves cursor to the top left
        print!("\x1B[2J\x1B[1;1H");

        // convert to ascii
        let output = convert_to_ascii(&grayscale_image);
        println!("{}",output);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
