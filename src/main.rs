use std::fs::File;
use std::io::Write;
use std::vec;
use image::{DynamicImage, ImageReader, GenericImageView};

// steps in convering image to ascii
// 1. resize
// 2. convert to grayscale
// 3. map brightness to a character
// 4. writing the ascii to txt

fn resize_image(image: DynamicImage) -> DynamicImage {
    let res = image.resize(80, 40, image::imageops::FilterType::Nearest);
    res
}

fn convert_to_grayscale(image: DynamicImage) -> DynamicImage {
    let gray = image.grayscale();
    gray
}

fn main() {
    let mut _img = ImageReader::open("images/red-moon.png").unwrap().decode().unwrap();

    // resizing image
    let resized_image = resize_image(_img);

    // converting to grayscale
    let grayscale_image = convert_to_grayscale(resized_image);

    // saving the grayscale just to check if it works
    grayscale_image.save("images/grayscale.png").unwrap();

    // mapping character to brightness
    let ramp = " .^*+=#&$@";
    let mut ascii = Vec::new();
    let (width, height) = grayscale_image.dimensions();
    for row in 0..height {
        let mut cur = String::new();
        for col in 0..width {
            let pixel = grayscale_image.get_pixel(col, row);
            let brightness = pixel[0];
            let idx = brightness as u32 * 9 / 255;
            cur.push(ramp.chars().nth(idx as usize).unwrap_or(' '));
        }
        cur.push_str("\n");
        ascii.push(cur);
    }

    let output = ascii.join("");
    let mut file = File::create("outputs/ascii.txt").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
