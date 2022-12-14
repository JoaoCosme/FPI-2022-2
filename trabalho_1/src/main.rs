use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Pixel, Rgb, Rgba};
const IMAGE_SIZE: usize = 1000;
const COLOR_NUMBER: usize = 256;
use plotters::prelude::*;

fn main() {
    let img = image::open("./src/test_images/Gramado_22k.jpg").expect("Should open image");

    let (w, h) = img.dimensions();
    let mut output = ImageBuffer::new(w, h);
    let mut gray_image = [[0; IMAGE_SIZE]; IMAGE_SIZE];

    for (x, y, pixel) in img.pixels() {
        let pixels = pixel.to_rgb().0;
        let gray_value = to_grayscale(&pixels);
        let x = x as usize;
        let y = y as usize;
        gray_image[x][y] = gray_value;
        output.put_pixel(
            x as u32,
            y as u32,
            Rgb {
                0: [gray_value, gray_value, gray_value],
            },
        );
    }
    let hist = make_histogram(&output);
    draw_histogram(&hist);
    output.save("./image.jpeg").expect("Should save image");
}

fn to_grayscale(pixels: &[u8; 3]) -> u8 {
    let red = pixels[0] as f64;
    let green = pixels[1] as f64;
    let blue = pixels[2] as f64;

    let new_val = 0.299 * red + 0.587 * green + 0.114 * blue;
    let new_val = new_val as u8;
    return new_val;
}

fn make_histogram(gray_image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> [usize; COLOR_NUMBER] {
    let mut histogram: [usize; COLOR_NUMBER] = [0; COLOR_NUMBER];
    for x in 0..gray_image.width() {
        for y in 0..gray_image.height() {
            let value = gray_image.get_pixel(x, y).to_rgb().0[0];
            histogram[value as usize] += 1;
        }
    }
    return histogram;
}

fn draw_histogram(histogram: &[usize; COLOR_NUMBER]) {
    let max_bin_value = histogram.iter().cloned().fold(0 as usize, usize::max);
    let root_area = BitMapBackend::new("./hist.jpeg", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Histograma", ("sans-serif", 40))
        .build_cartesian_2d((0..COLOR_NUMBER).into_segmented(), 0..max_bin_value)
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
