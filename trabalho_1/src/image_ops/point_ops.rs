use super::super::COLOR_NUMBER;

use image::Rgb;

use image::ImageBuffer;

pub(crate) fn make_gray_image(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let mut output = ImageBuffer::new(width, height);

    let (width, height) = image.dimensions();
    for x in 0..width {
        for y in 0..height {
            let gray_pixel = to_grayscale(&image.get_pixel(x, y).0);
            output.put_pixel(
                x,
                y,
                Rgb {
                    0: [gray_pixel, gray_pixel, gray_pixel],
                },
            );
        }
    }
    return output;
}

pub(crate) fn to_grayscale(pixels: &[u8; 3]) -> u8 {
    let red = pixels[0] as f64;
    let green = pixels[1] as f64;
    let blue = pixels[2] as f64;

    let new_val = 0.299 * red + 0.587 * green + 0.114 * blue;
    let new_val = new_val as u8;
    return new_val;
}

pub(crate) fn make_histogram(gray_image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> [usize; COLOR_NUMBER] {
    let mut histogram: [usize; COLOR_NUMBER] = [0; COLOR_NUMBER];
    for x in 0..gray_image.width() {
        for y in 0..gray_image.height() {
            let value = gray_image.get_pixel(x, y).to_rgb().0[0];
            histogram[value as usize] += 1;
        }
    }
    return histogram;
}

pub(crate) fn horizontal_flip(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = image.width();
    let half = width / 2;
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, image.height());
    for x in 0..half {
        for y in 0..image.height() {
            output.put_pixel(x, y, image.get_pixel(width - x - 1 as u32, y).clone());
            output.put_pixel(width - x - 1, y as u32, image.get_pixel(x, y).clone());
        }
    }
    return output;
}

pub(crate) fn vertical_flip(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = image.width();
    let height = image.height();
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, image.height());
    for x in 0..width {
        for y in 0..height / 2 {
            output.put_pixel(x, y, image.get_pixel(x, height - 1 - y).clone());
            output.put_pixel(x, height - y - 1 as u32, image.get_pixel(x, y).clone());
        }
    }
    return output;
}
