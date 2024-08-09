use std::process::Output;
use image::{open, GenericImageView, ImageFormat, DynamicImage, imageops::FilterType, imageops::crop_imm, ImageBuffer, Rgba};
use rayon::prelude::*;
use std::{env, path::Path, fs};

fn load_image(filepath: &str) -> Result<DynamicImage, image::ImageError>{
    image::open(filepath)
}

fn resize_image(image: &DynamicImage, width: u32, height: u32) -> DynamicImage{
    image.resize(width, height, FilterType::Lanczos3)
}

fn save_image(image: &DynamicImage, output_path: &str) {
    image.save_with_format(output_path, ImageFormat::Png).expect("Failed to save image");
}

fn rotate_image(img: &DynamicImage, degrees: u32) -> DynamicImage{
    match degrees {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => {
            eprintln!("Unsupported rotation angle!");
            img.clone()
        }
    }
}


fn resize_image_maintain_ratio(img: &DynamicImage, new_width: Option<u32>, new_height: Option<u32>) -> DynamicImage {
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

fn custom_rotate_90_clockwise(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,) -> ImageBuffer<Rgba<u8>, Vec<u8>>{
    let (width, height) = img.dimensions();
    let mut new_img = ImageBuffer::new(height, width); // dimensions swapped
    img.enumerate_pixels().for_each(|(x, y, pixel)| {
        let new_x = height - y - 1;
        let new_y = x;
        new_img.put_pixel(new_x, new_y, *pixel);
    });
    new_img
}

fn crop_image(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage{
    let cropped_image = crop_imm(img, x, y, width, height);
    DynamicImage::ImageRgba8(cropped_image.to_image())
}



fn process_image(operation: &str, path: &Path) {
    let img= load_image(path.to_str().expect("oops")).unwrap();
    let route = path.parent().unwrap().to_str().unwrap();
    let extension = "png";
    let outpath = format!(
        "{}/{}_{}.{}",
        route, operation, path.file_stem().unwrap().to_str().unwrap(), extension
    );
    match operation {
        "crop" => {
            let cropped = crop_image(&img, 50, 500, 500, 500);
            save_image(&cropped, &outpath);
        },
        "rotate" => {
            let rotated_image = rotate_image(&img, 270);
            save_image(&rotated_image, &outpath);
        },
        "resize" => {
            let resized_image = resize_image(&img, 512, 512);
            save_image(&resized_image, &outpath);
        },
        "resize-ratio" => {
            let resized_ratio = resize_image_maintain_ratio(&img, Some(512), Some(246));
            save_image(&resized_ratio, &outpath);
        },
        "to-png" => {
            img.save_with_format(
                outpath, ImageFormat::Png).expect("Save to save as PNG");
        },
        "custom_rotate" => {
            let rotated_img = custom_rotate_90_clockwise(
                &img.to_rgba8());
            rotated_img.save(outpath).expect("Failed to rotate");
        },
        _ => {
            eprintln!(
                "We only support: [crop, rotate, resize, resize-ratio, formats, to-png, custom_rotate]");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation = &args[1];
    let path = Path::new(&args[2]);

    if path.is_dir(){
        fs::read_dir(path)
            .expect("Dir not found!")
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "jpg" || ext == "png"))
            .par_bridge()
            .for_each(|entry| {
                process_image(operation, &entry.path());
            });
    } else {
        process_image(operation, path);
    }
}
