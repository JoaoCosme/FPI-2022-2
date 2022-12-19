use std::ops::Div;

use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb};
const COLOR_NUMBER: usize = 256;
use fltk::{
    app,
    button::Button,
    dialog::{self, FileChooser, FileChooserType, FileDialog},
    frame::Frame,
    image::SharedImage,
    input::Input,
    prelude::*,
    window::Window,
};
const SAVED_FILE: &'static str = "./loaded_image.jpeg";

fn main() {
    make_ui();
}

fn pick_file() {
    let mut file_chooser = FileChooser::new(
        ".",
        "*.{jpeg,jpg}",
        FileChooserType::Single,
        "Select a File!",
    );
    file_chooser.show();
    while file_chooser.shown() {
        app::wait();
    }
    image::open(file_chooser.value(0).expect("Should have choosen file"))
        .expect("Should open image")
        .save(SAVED_FILE)
        .expect("Should save opened image");
}

fn make_ui() {
    pick_file();
    let img = image::open(SAVED_FILE).expect("Should open image");
    let (width, height) = img.dimensions();
    let window_width = (width + 40).max(500) as i32;
    let window_height = (height+ 20).max(400) as i32;
    let width = width as i32;
    let height = height as i32;
    let app = app::App::default();
    let mut window = Window::new(0, 0, window_width, window_height + 50, "Base Image");
    let mut frame = Frame::new(20, 10, width, height, "");
    let mut image = SharedImage::load(SAVED_FILE).unwrap();
    image.scale(width, height, true, true);
    frame.set_image(Some(image));
    let mut but_equalize = Button::default()
        .with_size((window_width-100) / 5, 20)
        .below_of(&frame, 0)
        .with_label("Equalize");
    let mut but_horizontal = Button::default()
        .size_of(&but_equalize)
        .right_of(&but_equalize, 5)
        .with_label("Flip Horizontal");
    let mut but_vertical = Button::default()
        .size_of(&but_equalize)
        .right_of(&but_horizontal, 5)
        .with_label("Flip Vertical");
    let mut but_gray = Button::default()
        .size_of(&but_equalize)
        .right_of(&but_vertical, 5)
        .with_label("Gray Scale");
    let mut save_result = Button::default()
        .size_of(&but_equalize)
        .right_of(&but_gray, 5)
        .with_label("Save Result");

    let mut equalize_val = Input::default()
        .size_of(&but_equalize)
        .below_of(&but_equalize, 1);

    equalize_val.set_value("0");

    but_horizontal.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        horizontal_flip(&img)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32);
    });
    but_gray.set_callback(move |_| {
        let img = image::open(SAVED_FILE).expect("Should open image");
        make_gray_image(&img)
        .save(SAVED_FILE)
        .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32);
    });
    but_vertical.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        vertical_flip(&img)
        .save(SAVED_FILE)
        .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32);
    });
    but_equalize.set_callback(move |_| {
        let img = image::open(SAVED_FILE).expect("Should open image");
        equalize_image(
            &img,
            equalize_val
                .value()
                .trim()
                .parse()
                .expect("Should have number!"),
        )
        .save(SAVED_FILE)
        .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32);
    });
    save_result.set_callback(move |_| {
        let img = image::open(SAVED_FILE).expect("Should open image");
        let mut save = FileDialog::new(dialog::FileDialogType::BrowseSaveFile);

        save.show();
        while Some(save.filename()).is_none() {
            app::wait();
        }
        img.save(save.filename())
            .expect("Should save image correctly");
    });
    window.make_resizable(false);
    window.show();
    app.run().ok();
}

fn update_frame(width: i32, height: i32) {
    let window_width = (width + 100).max(500) as i32;
    let window_height = (height).max(400) as i32;
    let width = width as i32;
    let height = height as i32;
    let mut window = Window::new(window_width, 0, window_width, window_height + 50, "Result");
    let mut frame = Frame::new(0, 0, width + 100, height, "").center_of_parent();
    let mut image = SharedImage::load(SAVED_FILE).unwrap();
    image.scale(width, height, true, true);
    frame.set_image(Some(image));
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

fn horizontal_flip(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
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

fn vertical_flip(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
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

fn equalize_image(image: &DynamicImage, num_of_colors: i32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
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
