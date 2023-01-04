use super::COLOR_NUMBER;

use image::Rgb;
use plotters::prelude::*;


use image::ImageBuffer;

use image::Pixel;
use show_image::winit::platform::unix::x11::ffi::XI_RawKeyReleaseMask;
use std::ops::Div;

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

pub(crate) fn equalize_image(image: &ImageBuffer<Rgb<u8>, Vec<u8>>, num_of_colors: i32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let image_gray = make_gray_image(&image);
    let hist = make_histogram(&image_gray);
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

pub fn apply_point_operation(image:&ImageBuffer<Rgb<u8>,Vec<u8>>, a: f32, b:f32) -> ImageBuffer<Rgb<u8>,Vec<u8>>{
    let width = image.width();
    let height = image.height();
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, image.height());
    for x in 0..width {
        for y in 0..height{
            let result = image.get_pixel(x, y).map(|pixel| -> u8 { ((pixel as f32 * a) + b) as u8});
            output.put_pixel(x, y, result);        
        }
    }
    return output;   
}


pub fn draw_histogram(histogram:&[usize;256], path:&'static str){
    let maxY = histogram.iter().cloned().fold(0 as usize, usize::max);
    let root_area = BitMapBackend::new(path,(600,400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Histograma",("sans-serif",40))
    .build_cartesian_2d((0..256).into_segmented(), 0..maxY)
    .unwrap();
  
    ctx.configure_mesh().draw().unwrap();
  
    ctx.draw_series((0..).zip(histogram.iter()).map(|(x,y)|{
      let x0 = SegmentValue::Exact(x);
      let x1 = SegmentValue::Exact(x+1);
      let mut bar = Rectangle::new([(x0,0),(x1,*y)],BLUE.filled());
      bar.set_margin(0, 0, 5, 5);
      bar
    })).unwrap();
}

pub fn apply_conv(kernel:[[f32;3];3],image:&ImageBuffer<Rgb<u8>,Vec<u8>>) -> ImageBuffer<Rgb<u8>,Vec<u8>>{
    let width = image.width();
    let height = image.height();
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, image.height());
    for x in 1..width-2 {
        for y in 1..height-2{
            let mut result = [0,0,0];
            let mut a = 0;
            for i in 0..=2{
                for j in 0..=2{
                    let disloc_x = i as i32-1;
                    let disloc_y = j as i32-1;
                    a += 1;

                    let pixel = image.get_pixel((x as i32 +disloc_x) as u32, (y as i32+disloc_y) as u32);
                    
                    result[0] = result[0] + (pixel[0] as f32 * kernel[i as usize][j as usize]).max(0.0) as u8 ;
                    result[1] = result[1] + (pixel[1] as f32 * kernel[i as usize][j as usize]).max(0.0) as u8 ;
                    result[2] = result[2] + (pixel[2] as f32 * kernel[i as usize][j as usize]).max(0.0) as u8 ;
                }
            }
            dbg!(a);
            output.put_pixel(x, y, Rgb((result)));        
        }
    }
    return output;   
}
