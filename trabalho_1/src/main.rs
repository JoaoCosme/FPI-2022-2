mod point_ops;

use fltk::{app, button::Button, frame::Frame, image::SharedImage, prelude::*, window::Window};
use image::GenericImageView;
use point_ops::{*, point_ops::{vertical_flip, draw_histogram, make_histogram, make_gray_image, horizontal_flip}};

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