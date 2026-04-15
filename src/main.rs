use std::fs::File;
use std::io::Write;
use image::{DynamicImage, ImageReader, GenericImageView};

// steps in convering image to ascii
// 1. resize
// 2. convert to grayscale
// 3. map brightness to a character
// 4. writing the ascii to txt

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
        let mut cur = String::new();
        for col in 0..width {
            let pixel = grayscale_image.get_pixel(col, row);
            let brightness = pixel[0];
            let idx = brightness as u32 * (ramp_len - 1) / 255;
            cur.push(ramp[idx as usize]);
        }
        cur.push_str("\n");
        output.push_str(&cur);
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

    // saving the grayscale just to check if it works
    // grayscale_image.save("outputs/grayscale.png").unwrap();

    // creating a file and writing the ascii to it
    let mut file = File::create("outputs/ascii.txt").unwrap();
    file.write_all(convert_to_ascii(&grayscale_image).as_bytes()).unwrap();

    println!("completed");
}
