use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb};
const COLOR_NUMBER: usize = 256;
use fltk::{app, button::Button, frame::Frame, image::SharedImage, prelude::*, window::Window};
use plotters::prelude::*;

fn main() {
    let img = image::open("./src/test_images/Underwater_53k.jpg").expect("Should open image");
    let (width, height) = img.dimensions();
    let width = width as i32;
    let height = height as i32;
    make_ui(width, height);
}

fn make_ui(width: i32, height: i32) {
    let app = app::App::default();
    let mut window = Window::new(0, 0, width, height + 25, "Hello world!");
    let mut frame = Frame::new(0, 0, width, height, "");
    let mut image = SharedImage::load("./src/test_images/Underwater_53k.jpg").unwrap();
    image.scale(width, height, true, true);
    frame.set_image(Some(image));
    let mut but_histogram = Button::default()
        .with_size((width - 10) / 4, 20)
        .below_of(&frame, 0)
        .with_label("Calculate Histogram");
    let mut but_horizontal = Button::default()
        .size_of(&but_histogram)
        .right_of(&but_histogram, 5)
        .with_label("Flip Horizontal");
    let mut but_vertical = Button::default()
        .size_of(&but_histogram)
        .right_of(&but_horizontal, 5)
        .with_label("Flip Vertical");
    let mut but_gray = Button::default()
        .size_of(&but_histogram)
        .right_of(&but_vertical, 5)
        .with_label("Gray Scale");
    but_horizontal.set_callback(move |_| {
        let img = image::open("./src/test_images/Underwater_53k.jpg")
            .expect("Should open image")
            .into_rgb8();
        horizontal_flip(&img)
            .save("./image.jpeg")
            .expect("Should save image");
        update_frame(width, height);
    });
    but_gray.set_callback(move |_| {
        let img = image::open("./src/test_images/Underwater_53k.jpg").expect("Should open image");
        make_gray_image(&img)
            .save("./image.jpeg")
            .expect("Should save image");
        update_frame(width, height);
    });
    but_vertical.set_callback(move |_| {
        let img = image::open("./src/test_images/Underwater_53k.jpg").expect("Should open image").into_rgb8();
        vertical_flip(&img)
            .save("./image.jpeg")
            .expect("Should save image");
        update_frame(width, height);
    });
    but_histogram.set_callback(move |_| {
        let img = image::open("./src/test_images/Underwater_53k.jpg").expect("Should open image");
        draw_histogram(&make_histogram(&make_gray_image(&img)),"./image.jpeg".to_string());
        update_frame(width, height);
    });
    window.make_resizable(false);
    window.show();
    app.run().ok();
}

fn update_frame(width: i32, height: i32) {
    let mut window = Window::new(width, 0, width, height + 25, "Result");
    let mut frame = Frame::new(0, 0, width, height, "");
    let mut image2 = SharedImage::load("./image.jpeg").unwrap();
    image2.scale(width, height, true, true);
    frame.set_image(Some(image2));
    window.show();
}

fn make_gray_image(img: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, h) = img.dimensions();
    let mut output = ImageBuffer::new(width, h);
    let mut gray_image: Vec<Vec<u8>> = vec![];

    let (width, height) = img.dimensions();
    for x in 0..width {
        let mut line = vec![];
        for y in 0..height {
            let gray_pixel = to_grayscale(&img.get_pixel(x, y).to_rgb().0);
            line.push(gray_pixel);
            output.put_pixel(
                x,
                y,
                Rgb {
                    0: [gray_pixel, gray_pixel, gray_pixel],
                },
            );
        }
        gray_image.push(line)
    }
    return output;
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

fn draw_histogram(histogram: &[usize; COLOR_NUMBER],save_path:String) {
    let max_bin_value = histogram.iter().cloned().fold(0 as usize, usize::max);
    let root_area = BitMapBackend::new(save_path.as_str(), (600, 400)).into_drawing_area();
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

fn horizontal_flip(gray_image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = gray_image.width();
    let half = width / 2;
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, gray_image.height());
    for x in 0..half {
        for y in 0..gray_image.height() {
            output.put_pixel(x, y, gray_image.get_pixel(width - x - 1 as u32, y).clone());
            output.put_pixel(width - x - 1, y as u32, gray_image.get_pixel(x, y).clone());
        }
    }
    return output;
}

fn vertical_flip(gray_image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = gray_image.width();
    let height = gray_image.height();
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, gray_image.height());
    for x in 0..width {
        for y in 0..height / 2 {
            output.put_pixel(x, y, gray_image.get_pixel(x, height - 1 - y).clone());
            output.put_pixel(x, height - y - 1 as u32, gray_image.get_pixel(x, y).clone());
        }
    }
    return output;
}
