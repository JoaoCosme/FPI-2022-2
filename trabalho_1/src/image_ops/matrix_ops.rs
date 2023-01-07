use image::Rgb;

use image::ImageBuffer;

pub fn apply_conv(
    kernel: [[f32; 3]; 3],
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    should_clamp: bool,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = image.width();
    let height = image.height();
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, image.height());
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let mut result = [0.0, 0.0, 0.0];
            for i in 0..=2 {
                for j in 0..=2 {
                    let disloc_x = i as i32 - 1;
                    let disloc_y = j as i32 - 1;

                    let pixel =
                        image.get_pixel((x as i32 + disloc_x) as u32, (y as i32 + disloc_y) as u32);

                    result[0] += pixel[0] as f32 * kernel[i as usize][j as usize];
                    result[1] += pixel[1] as f32 * kernel[i as usize][j as usize];
                    result[2] += pixel[2] as f32 * kernel[i as usize][j as usize];
                }
            }

            if should_clamp {
                result[0] += 127.0;
                result[1] += 127.0;
                result[2] += 127.0;
            }

            result[0] = adjust_pixel_value(result[0]);
            result[1] = adjust_pixel_value(result[1]);
            result[2] = adjust_pixel_value(result[2]);

            if result[0] > 255.0 || result[0] < 0.0 {
                panic!("Incorrect value!");
            }

            output.put_pixel(
                x,
                y,
                Rgb([result[0] as u8, result[1] as u8, result[2] as u8]),
            );
        }
    }
    return output;
}

pub(self) fn adjust_pixel_value(pixel: f32) -> f32 {
    return if pixel > 255.0 {
        255.0
    } else {
        if pixel < 0.0 {
            0.0
        } else {
            pixel
        }
    };
}
