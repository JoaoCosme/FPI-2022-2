use std::ops::Div;

use image::Pixel;
use plotters::prelude::*;

use image::Rgb;

use image::ImageBuffer;

use crate::COLOR_NUMBER;

use super::point_ops;
use super::point_ops::make_gray_image;
use super::point_ops::make_histogram;

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

    let hist_cumulative = calculate_cumulative_histogram(&image, hist);

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

pub(crate) fn fixed_equalize_image(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let hist = point_ops::make_histogram(&make_gray_image(image));
    let image = image;
    let width = image.width();
    let height = image.height();
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, image.height());

    let hist_cumulative = calculate_cumulative_histogram(&image, hist);

    for x in 0..width {
        for y in 0..height {
            let rgb = image.get_pixel(x, y).to_rgb().0;
            output.put_pixel(
                x,
                y,
                Rgb {
                    0: [
                        hist_cumulative[rgb[0] as usize] as u8,
                        hist_cumulative[rgb[1] as usize] as u8,
                        hist_cumulative[rgb[2] as usize] as u8,
                    ],
                },
            );
        }
    }
    return output;
}

pub(crate) fn calculate_cumulative_histogram(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    hist: [usize; COLOR_NUMBER],
) -> Vec<f32> {
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
    hist_cumulative
}

pub fn match_histogram(
    base_image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    image_to_match: &ImageBuffer<Rgb<u8>, Vec<u8>>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = base_image.width();
    let height = base_image.height();
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let base_gray_image = &make_gray_image(&base_image);
    let base_cumulative_histogram = calculate_cumulative_histogram(
        base_image,
        make_histogram(&(&make_gray_image(&base_image))),
    );

    let gray_image_to_match = make_gray_image(&image_to_match);
    let cumulative_histogram_to_match =
        calculate_cumulative_histogram(image_to_match, make_histogram(&gray_image_to_match));

    let mut matched_histogram: [usize; COLOR_NUMBER] = [0; COLOR_NUMBER];

    for shade_in_base in 0..=255 {
        let base_cumulative_value = base_cumulative_histogram[shade_in_base];
        for shade_in_target in 0..=255 {
            let target_cumulative_value = cumulative_histogram_to_match[shade_in_target];

            if base_cumulative_value == target_cumulative_value {
                matched_histogram[shade_in_base] = shade_in_target;
                break;
            } else if base_cumulative_value < target_cumulative_value {
                if shade_in_target == 0 {
                    matched_histogram[shade_in_base] = 0;
                } else if shade_in_target as f32 - base_cumulative_value
                    <= base_cumulative_value - (shade_in_target - 1) as f32
                {
                    matched_histogram[shade_in_base] = shade_in_target;
                } else {
                    matched_histogram[shade_in_base] = shade_in_target - 1;
                }
                break;
            }
        }
    }

    for x in 0..width {
        for y in 0..height {
            let value = base_gray_image.get_pixel(x, y).to_rgb().0[0];
            let color = matched_histogram[value as usize] as u8;
            output.put_pixel(
                x,
                y,
                Rgb {
                    0: [color, color, color],
                },
            )
        }
    }

    output
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
