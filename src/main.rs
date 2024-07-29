use std::process::Output;
use image::{open, GenericImageView, ImageFormat, DynamicImage, imageops::FilterType};

fn resize_image(path: &str, width: u32, height: u32) -> DynamicImage{
    let image = open(path).expect("Failed to open image");
    image.resize(width, height, FilterType::Lanczos3)
}

fn save_image(image: &DynamicImage, output_path: &str) {
    image.save_with_format(output_path, ImageFormat::Png).expect("Failed to save image");
}

fn rotate_image(path: &str, degrees: u32) -> DynamicImage{
    let img = open(path).expect("Failed to load image");
    match degrees {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => {
            eprintln!("Unsupported rotation angle!");
            img
        }
    }
}


fn main() {
    let path = "assets/sample_img.jpg";
    // let outpath = "assets/resized.png";
    let outrotatepath = "assets/rotated.png";

    // let resized_image = resize_image(path, 512, 512);
    let rotated_image = rotate_image(path, 270);
    save_image(&rotated_image, outrotatepath);
    // let img = open("assets/sample_img.jpg").expect("Failure opening image");
    //     let (width, height) = img.dimensions();
    // img.save_with_format(
    //     "assets/sample_img.png", ImageFormat::Png).expect("Save to save as PNG");
    // img.save_with_format(
        // "assets/sample_img.webp", ImageFormat::WebP).expect("Save to save as WebP");

}
