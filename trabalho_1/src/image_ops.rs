use image::Rgb;

use image::ImageBuffer;

use image::Pixel;

pub mod histogram_ops;
pub mod matrix_ops;
pub mod point_ops;

pub fn apply_point_operation(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    a: f32,
    b: f32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = image.width();
    let height = image.height();
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, image.height());
    for x in 0..width {
        for y in 0..height {
            let result = image
                .get_pixel(x, y)
                .map(|pixel| -> u8 { ((pixel as f32 * a) + b) as u8 });
            output.put_pixel(x, y, result);
        }
    }
    return output;
}
