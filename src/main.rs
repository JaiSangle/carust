use image::{DynamicImage, ImageReader};

// steps in convering image to ascii
// 1. resize
// 2. convert to grayscale
// 3. map brightness to a character
// 4. print row by row

fn resize_image(image: DynamicImage) -> DynamicImage {
    let res = image.resize(80, 40, image::imageops::FilterType::Nearest);
    res
}

fn convert_to_grayscale(image: DynamicImage) -> DynamicImage {
    let gray = image.grayscale();
    gray
}

// what we may need apart from the image crate
// reading file and writing into a txt file for the final ascii art

fn main() {
    let mut _img = ImageReader::open("images/red-moon.png").unwrap().decode().unwrap();

    // resizing image
    let resized_image = resize_image(_img);

    // converting to grayscale
    let grayscale_image = convert_to_grayscale(resized_image);

    // saving the grayscale just to check if it works
    grayscale_image.save("images/grayscale.png").unwrap();
    print!("completed");

    // mapping character to brightness
}
