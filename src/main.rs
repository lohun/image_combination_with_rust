pub mod args;
pub mod floating_image;
pub mod image_data_error;
use args::Args;
use floating_image::FloatingImage;
use image_data_error::ImageDataError;
use image::imageops::FilterType::Triangle;
use image::{io::Reader, DynamicImage, ImageFormat, GenericImageView};
use std::io::BufReader;
use std::fs::File;

fn main()-> Result<(), ImageDataError> {
    let args = Args::new();

    let (image_2, image_format_2) = find_image_from_path(args.image2);
    let ( image_1, image_format_1 ) = find_image_from_path(args.image1);

    let (width, height) = get_image_dimensions(image_1.dimensions(), image_2.dimensions());

    if image_format_1 != image_format_2 {
      return Err(ImageDataError::DifferentImageFormats);
    }

    let (image_1, image_2) = resize_image(image_1, image_2, (width, height));

    let mut floating_image = FloatingImage::new(width, height, &args.output);

    let combined_data = combine_images(image_1, image_2);
    floating_image
        .set_data(combined_data)
        .expect("There was an error");

    image::save_buffer_with_format(
        floating_image.name,
        &floating_image.data,
        floating_image.width,
        floating_image.height,
        image::ColorType::Rgba8,
        image_format_1,
    ).expect("Final Error");
    Ok(())
}

fn get_image_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  print!("{:?}", dim_1);
    let pix1 = dim_1.0 * dim_1.1;
    let pix2 = dim_2.0 * dim_2.1;
    let result: (u32, u32) = if pix1 < pix2 {dim_1} else {dim_2};
    result
}

fn resize_image(
    image_1: DynamicImage,
    image_2: DynamicImage,
    size: (u32, u32),
) -> (DynamicImage, DynamicImage) {
    let (width, height) = size;
    (
        image_1.resize_exact(width, height, Triangle),
        image_2.resize_exact(width, height, Triangle),
    )
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    print!("{:?}", image);
    (image, image_format)
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let length = vec_1.len();
    let mut combined_data = vec![0u8; length];

    let mut i = 0;
    while i < length {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }

        i += 4;
    }
    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds"),
        };
        rgba.push(val)
    }
    rgba
}
// program to build image combiner
