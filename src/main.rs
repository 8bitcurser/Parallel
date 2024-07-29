use std::process::Output;
use image::{open, GenericImageView, ImageFormat, DynamicImage, imageops::FilterType, imageops::crop_imm};

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


fn resize_image_maintain_ratio(path: &str, new_width: Option<u32>, new_height: Option<u32>) -> DynamicImage {
    let img = open(path).expect("Failed to open image");
    let (width, height) = img.dimensions();
    // calculate new dimensions while maintaining aspect ratio
    let ratio = width as f32 / height as f32;
    let (resize_width, resize_height) = match(new_width, new_height) {
        (Some(w), None) => (w, (width as f32 / ratio).round() as u32),
        (None, Some(h)) => ((height as f32 / ratio).round() as u32, h),
        (Some(w), Some(h)) => (w, h),
        (None, None) => (width, height)
    };

    img.resize(resize_width, resize_height, FilterType::Lanczos3)
}


fn crop_image(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage{
    let cropped_image = crop_imm(img, x, y, width, height);
    DynamicImage::ImageRgba8(cropped_image.to_image())
}


fn main() {
    let path = "assets/sample_img.jpg";
    // let outpath = "assets/resized.png";
    // let outrotatepath = "assets/rotated.png";
    let outpath = "assets/resized_ratio.png";
    let croppedpath = "assets/cropped.png";

    // let resized_ratio = resize_image_maintain_ratio(path, Some(512), Some(246));
    // save_image(&resized_ratio, outpath);

    let img = open(path).expect("Oops!");
    let crop_im = crop_image(&img, 50, 500, 500, 500);
    save_image(&crop_im, croppedpath);
    // let resized_image = resize_image(path, 512, 512);
    // let rotated_image = rotate_image(path, 270);
    // save_image(&rotated_image, outrotatepath);

    // let img = open("assets/sample_img.jpg").expect("Failure opening image");
    //     let (width, height) = img.dimensions();
    // img.save_with_format(
    //     "assets/sample_img.png", ImageFormat::Png).expect("Save to save as PNG");
    // img.save_with_format(
        // "assets/sample_img.webp", ImageFormat::WebP).expect("Save to save as WebP");

}
