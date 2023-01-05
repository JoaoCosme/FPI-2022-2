mod image_ops;
mod kernel;
mod matrix_ops;
mod test;
use image::GenericImageView;
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
const COPIED_FILE: &'static str = "./copy.jpeg";
const HISTOGRAM: &'static str = "./histogram.jpeg";

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
    let dynamic_image = image::open(file_chooser.value(0).expect("Should have choosen file"))
        .expect("Should open image");
    dynamic_image
        .save(SAVED_FILE)
        .expect("Should save opened image");
    dynamic_image
        .save(COPIED_FILE)
        .expect("Should save opened image");
}

fn make_ui() {
    pick_file();
    let img = image::open(SAVED_FILE).expect("Should open image");
    let (width, height) = img.dimensions();
    let window_width = calc_window_width(width);
    let window_height = calc_window_height(height);
    let width = width as i32;
    let height = height as i32;
    let app = app::App::default();
    let mut window = Window::new(0, 0, window_width, window_height + 50, "Base Image");
    let mut frame = Frame::new(20, 10, width, height, "");
    let mut image = SharedImage::load(SAVED_FILE).unwrap();
    image.scale(width, height, true, true);
    frame.set_image(Some(image));
    let mut but_equalize = Button::default()
        .with_size((window_width - 100) / 5, 20)
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
    let mut but_bright = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .below_of(&but_horizontal, 0)
        .with_label("Bright Up");
    let mut but_contrast = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_bright, 5)
        .with_label("Contrast Up");
    let mut but_negative = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_contrast, 5)
        .with_label("Negative");
    let mut but_histogram = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_negative, 5)
        .with_label("Histogram");
    let mut but_laplacian = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .below_of(&but_histogram, 5)
        .with_label("LaPlacian");
    let mut but_gauss = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .below_of(&equalize_val, 5)
        .with_label("Gaussian");
    let mut but_passa_alta = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_gauss, 5)
        .with_label("Passa Alta");
    let mut but_pw_hx = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_passa_alta, 5)
        .with_label("Prewitt Hx");
    let mut but_pw_hy = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_pw_hx, 5)
        .with_label("Prewitt Hy");
    let mut but_sobel_hx = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .below_of(&but_gauss, 5)
        .with_label("Sobel Hx");
    let mut but_sobel_hy: Button = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_sobel_hx, 5)
        .with_label("Sobel Hy");

    let mut kernel_0 = Input::default()
        .size_of(&but_equalize)
        .below_of(&but_sobel_hx, 1);
    let mut kernel_1 = Input::default()
        .size_of(&but_equalize)
        .right_of(&kernel_0, 1);
    let mut kernel_2 = Input::default()
        .size_of(&but_equalize)
        .right_of(&kernel_1, 1);
    let mut kernel_3 = Input::default()
        .size_of(&but_equalize)
        .below_of(&kernel_0, 1);
    let mut kernel_4 = Input::default()
        .size_of(&but_equalize)
        .right_of(&kernel_3, 1);
    let mut kernel_5 = Input::default()
        .size_of(&but_equalize)
        .right_of(&kernel_4, 1);
    let mut kernel_6 = Input::default()
        .size_of(&but_equalize)
        .below_of(&kernel_3, 1);
    let mut kernel_7 = Input::default()
        .size_of(&but_equalize)
        .right_of(&kernel_6, 1);
    let mut kernel_8 = Input::default()
        .size_of(&but_equalize)
        .right_of(&kernel_7, 1);
    let mut but_custom_kernel: Button = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&kernel_8, 1)
        .with_label("Custom Kernel");

    let mut but_reset: Button = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_sobel_hy, 5)
        .with_label("Reset");

    equalize_val.set_value("0");
    kernel_0.set_value("0");
    kernel_1.set_value("0");
    kernel_2.set_value("0");
    kernel_3.set_value("0");
    kernel_4.set_value("0");
    kernel_5.set_value("0");
    kernel_6.set_value("0");
    kernel_7.set_value("0");
    kernel_8.set_value("0");

    but_horizontal.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::point_ops::horizontal_flip(&img)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_gray.set_callback(move |_| {
        let img = turn_image_to_grayscale();
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_vertical.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::point_ops::vertical_flip(&img)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_equalize.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        let num_of_colors = fetch_input_val(&equalize_val);
        image_ops::equalize_image(&img, num_of_colors)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width(), img.height(), SAVED_FILE);
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
    but_bright.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::apply_point_operation(&img, 1.0, 10.0)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_contrast.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::apply_point_operation(&img, 0.25, 0.0)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width(), img.height(), SAVED_FILE);
    });

    but_negative.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::apply_point_operation(&img, -1.0, 255.0)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_histogram.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::draw_histogram(
            &image_ops::point_ops::make_histogram(&image_ops::point_ops::make_gray_image(&img)),
            HISTOGRAM,
        );
        update_frame(img.width(), img.height(), HISTOGRAM);
    });

    but_gauss.set_callback(move |_| {
        apply_kernel_to_image(kernel::GAUSS, false, false);
    });

    but_laplacian.set_callback(move |_| {
        apply_kernel_to_image(kernel::LAPLACIAN, false, true);
    });

    but_passa_alta.set_callback(move |_| {
        apply_kernel_to_image(kernel::PASSA_ALTA, false, true);
    });

    but_pw_hx.set_callback(move |_| {
        apply_kernel_to_image(kernel::PREWITT_HX, true, true);
    });

    but_pw_hy.set_callback(move |_| {
        apply_kernel_to_image(kernel::PREWITT_HY, true, true);
    });

    but_sobel_hx.set_callback(move |_| {
        apply_kernel_to_image(kernel::SOBEL_HX, true, true);
    });

    but_sobel_hy.set_callback(move |_| {
        apply_kernel_to_image(kernel::SOBEL_HY, true, true);
    });
    but_reset.set_callback(move |_| {
        let img = image::open(COPIED_FILE).expect("Should open image");
        img.save(SAVED_FILE).ok();
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_custom_kernel.set_callback(move |_| {
        let custom_kernel = [
            [
                fetch_input_val(&kernel_0) as f32,
                fetch_input_val(&kernel_1) as f32,
                fetch_input_val(&kernel_2) as f32,
            ],
            [
                fetch_input_val(&kernel_3) as f32,
                fetch_input_val(&kernel_4) as f32,
                fetch_input_val(&kernel_5) as f32,
            ],
            [
                fetch_input_val(&kernel_6) as f32,
                fetch_input_val(&kernel_7) as f32,
                fetch_input_val(&kernel_8) as f32,
            ],
        ];
        apply_kernel_to_image(custom_kernel, false, true);
    });

    window.make_resizable(false);
    window.show();
    app.run().ok();
}

fn calc_window_height(height: u32) -> i32 {
    (height + 150).max(400) as i32
}

fn calc_window_width(width: u32) -> i32 {
    (width + 100).max(700) as i32
}

fn turn_image_to_grayscale() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let img = image::open(SAVED_FILE)
        .expect("Should open image")
        .into_rgb8();
    image_ops::point_ops::make_gray_image(&img)
        .save(SAVED_FILE)
        .expect("Should save image");
    img
}

fn apply_kernel_to_image(kernel: [[f32; 3]; 3], should_clamp: bool, turn_gray: bool) {
    if turn_gray {
        turn_image_to_grayscale();
    }
    let image = image::open(SAVED_FILE)
        .expect("Should open image")
        .into_rgb8();
    image_ops::apply_conv(kernel, &image, should_clamp)
        .save(SAVED_FILE)
        .expect("Should save image");
    update_frame(image.width(), image.height(), SAVED_FILE);
}

fn update_frame(width: u32, height: u32, file_path: &'static str) {
    let window_width = calc_window_width(width);
    let window_height = calc_window_height(height);
    let width = width as i32;
    let height = height as i32;
    let mut window = Window::new(window_width, 0, window_width, window_height + 50, "Result");
    let mut frame = Frame::new(0, 0, width + 100, height, "").center_of_parent();
    let mut image = SharedImage::load(file_path).unwrap();
    image.scale(width, height, true, true);
    frame.set_image(Some(image));
    window.show();
}

fn fetch_input_val(input: &Input) -> i32 {
    input.value().trim().parse().expect("Should have number!")
}
