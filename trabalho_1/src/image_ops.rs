use image::Rgb;
use plotters::prelude::*;

use image::ImageBuffer;

use image::Pixel;
use std::ops::Div;

pub mod point_ops;

pub(crate) fn equalize_image(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    num_of_colors: i32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let image_gray = point_ops::make_gray_image(&image);
    let hist = point_ops::make_histogram(&image_gray);
    let image = image_gray;
    let width = image.width();
    let height = image.height();
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, image.height());
    let alpha = 255.0.div(image.height().wrapping_mul(image.width()) as f32);

    let mut hist_cumulative = vec![];
    hist_cumulative.push(alpha * hist[0] as f32);
    for i in 1..=255 {
        hist_cumulative.push(
            hist_cumulative
                .last()
                .expect("Should have at least one value")
                + (alpha * hist[i as usize] as f32),
        );
    }
    let t1 = image.pixels().map(|pixel| pixel.0[0]).min().unwrap();
    let t2 = image.pixels().map(|pixel| pixel.0[0]).max().unwrap();
    let tam_int = t2 as i32 - t1 as i32 + 1;

    let should_adjust_bins = num_of_colors < tam_int;
    let tb = tam_int / num_of_colors;

    for x in 0..width {
        for y in 0..height {
            let value = image.get_pixel(x, y).to_rgb().0[0];
            let mut color = hist_cumulative[value as usize] as u8;
            if should_adjust_bins {
                for x in 0..num_of_colors {
                    let bin_start = t1 as f32 - 0.5 + (tb * x) as f32;
                    let bin_end = t1 as f32 - 0.5 + (tb * (x + 1)) as f32;
                    if (value as f32 >= bin_start) && (value as f32 <= bin_end) {
                        color = bin_start as u8;
                        break;
                    }
                    if x == num_of_colors - 1 {
                        color = bin_start as u8 - 1;
                    }
                }
            }
            output.put_pixel(
                x,
                y,
                Rgb {
                    0: [color, color, color],
                },
            );
        }
    }
    return output;
}

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

pub fn draw_histogram(histogram: &[usize; 256], path: &'static str) {
    let max_y = histogram.iter().cloned().fold(0 as usize, usize::max);
    let root_area = BitMapBackend::new(path, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Histograma", ("sans-serif", 40))
        .build_cartesian_2d((0..256).into_segmented(), 0..max_y)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series((0..).zip(histogram.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x + 1);
        let mut bar = Rectangle::new([(x0, 0), (x1, *y)], BLUE.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();
}

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
                    // dbg!(disloc_y,disloc_x,kernel[i as usize][j as usize],pixel);

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

            // dbg!(result);
            // return  output;
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
