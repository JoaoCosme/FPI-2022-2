mod image_ops;
mod kernel_repo;
use image::GenericImageView;
pub const COLOR_NUMBER: usize = 256;
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
use image_ops::histogram_ops::match_histogram;

const SAVED_FILE: &'static str = "./loaded_image.jpeg";
const COPIED_FILE: &'static str = "./copy.jpeg";
const HISTOGRAM: &'static str = "./histogram.jpeg";

fn main() {
    make_ui();
}

fn pick_file() {
    let mut file_chooser = fetch_file();
    let dynamic_image = image::open(file_chooser.value(0).expect("Should have choosen file"))
        .expect("Should open image");
    dynamic_image
        .save(SAVED_FILE)
        .expect("Should save opened image");
    dynamic_image
        .save(COPIED_FILE)
        .expect("Should save opened image");
}

fn fetch_file() -> FileChooser {
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
    file_chooser
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
    let mut image = SharedImage::load(SAVED_FILE).unwrap();
    let mut frame = Frame::new((window_width - image.width()) / 2, 10, width, height, "");
    image.scale(width, height, true, true);
    frame.set_image(Some(image));

    let button_width = (window_width - 100) / 5;
    let button_height = 20;

    let mut but_equalize = Button::default()
        .with_size(button_width, button_height)
        .with_pos(10, img.height() as i32 + 15)
        .with_label("Equalize");

    let mut equalize_val = Input::default()
        .size_of(&but_equalize)
        .below_of(&but_equalize, 1);
    let mut but_bright = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_equalize, 0)
        .with_label("Bright Up");
    let mut but_contrast = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_bright, 5)
        .with_label("Contrast Up");

    let mut but_negative = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_contrast, 5)
        .with_label("Negative");
    let mut but_save_result = Button::default()
        .size_of(&but_negative)
        .right_of(&but_negative, 5)
        .with_label("Save Result");
    let mut but_gray = Button::default()
        .size_of(&but_equalize)
        .below_of(&but_negative, 5)
        .with_label("Gray Scale");

    let mut but_reset: Button = Button::default()
        .with_size(button_width, button_height)
        .below_of(&but_save_result, 5)
        .with_label("Reset");

    let mut but_horizontal = Button::default()
        .size_of(&but_equalize)
        .below_of(&equalize_val, 5)
        .with_label("Flip Horizontal");
    let mut but_vertical = Button::default()
        .size_of(&but_equalize)
        .right_of(&but_horizontal, 5)
        .with_label("Flip Vertical");

    let mut but_rotate_left: Button = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_vertical, 5)
        .with_label("Rotate Left");
    let mut but_rotate_right: Button = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_rotate_left, 5)
        .with_label("Rotate Right");

    let mut but_zoom_in: Button = Button::default()
        .with_size(button_width, button_height)
        .below_of(&but_horizontal, 5)
        .with_label("Zoom in");
    let mut but_zoom_out: Button = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_zoom_in, 5)
        .with_label("Zoom out");

    let mut but_histogram = Button::default()
        .with_size(button_width, button_height)
        .below_of(&but_zoom_in, 5)
        .with_label("Histogram");

    let mut out_sx = Input::default()
        .size_of(&but_zoom_out)
        .right_of(&but_zoom_out, 1);
    let mut out_sy = Input::default().size_of(&out_sx).right_of(&out_sx, 1);
    let mut bright_val = Input::default()
        .size_of(&but_zoom_out)
        .below_of(&but_bright, 1);
    let mut contrast_val = Input::default()
        .size_of(&but_zoom_out)
        .below_of(&but_contrast, 1);

    let mut but_histogram_matching: Button = Button::default()
        .with_size(button_width, button_height)
        .below_of(&but_zoom_out, 5)
        .with_label("Match hist.");

    let mut but_equalize_fixed: Button = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_histogram_matching, 5)
        .with_label("Fixed equalize");

    let mut but_gauss = Button::default()
        .with_size(button_width, button_height)
        .below_of(&but_histogram, 5)
        .with_label("Gaussian");
    let mut but_passa_alta = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_gauss, 5)
        .with_label("Passa Alta");
    let mut but_pw_hx = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_passa_alta, 5)
        .with_label("Prewitt Hx");
    let mut but_pw_hy = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_pw_hx, 5)
        .with_label("Prewitt Hy");
    let mut but_laplacian = Button::default()
        .with_size(button_width, button_height)
        .right_of(&but_pw_hy, 5)
        .with_label("LaPlacian");
    let mut but_sobel_hx = Button::default()
        .with_size(button_width, button_height)
        .below_of(&but_gauss, 5)
        .with_label("Sobel Hx");
    let mut but_sobel_hy: Button = Button::default()
        .with_size(button_width, button_height)
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
        .with_size(button_width, button_height)
        .right_of(&kernel_8, 1)
        .with_label("Custom Kernel");

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
    out_sy.set_value("2");
    out_sx.set_value("2");
    bright_val.set_value("0");
    contrast_val.set_value("0");

    but_horizontal.set_callback(move |_| {
        apply_function_to_image(&image_ops::point_ops::horizontal_flip);
    });
    but_gray.set_callback(move |_| {
        let img = turn_image_to_grayscale();
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_vertical.set_callback(move |_| {
        apply_function_to_image(&image_ops::point_ops::vertical_flip);
    });
    but_equalize.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        let num_of_colors = fetch_input_val(&equalize_val) as i32;
        image_ops::histogram_ops::equalize_image(&img, num_of_colors)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_equalize_fixed.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::histogram_ops::fixed_equalize_image(&img)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_save_result.set_callback(move |_| {
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
        let mut b = fetch_input_val(&bright_val);
        normalize_bias(&mut b);
        image_ops::apply_point_operation(&img, 1.0, b)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_contrast.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        let mut a = fetch_input_val(&contrast_val);
        normalize_gain(&mut a);
        image_ops::apply_point_operation(&img, a, 0.0)
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
        image_ops::histogram_ops::draw_histogram(
            &image_ops::point_ops::make_histogram(&image_ops::point_ops::make_gray_image(&img)),
            HISTOGRAM,
        );
        update_frame(img.width(), img.height(), HISTOGRAM);
    });

    but_gauss.set_callback(move |_| {
        apply_kernel_to_image(kernel_repo::GAUSS, false, false);
    });

    but_laplacian.set_callback(move |_| {
        apply_kernel_to_image(kernel_repo::LAPLACIAN, false, true);
    });

    but_passa_alta.set_callback(move |_| {
        apply_kernel_to_image(kernel_repo::PASSA_ALTA, false, true);
    });

    but_pw_hx.set_callback(move |_| {
        apply_kernel_to_image(kernel_repo::PREWITT_HX, true, true);
    });

    but_pw_hy.set_callback(move |_| {
        apply_kernel_to_image(kernel_repo::PREWITT_HY, true, true);
    });

    but_sobel_hx.set_callback(move |_| {
        apply_kernel_to_image(kernel_repo::SOBEL_HX, true, true);
    });

    but_sobel_hy.set_callback(move |_| {
        apply_kernel_to_image(kernel_repo::SOBEL_HY, true, true);
    });
    but_reset.set_callback(move |_| {
        let img = image::open(COPIED_FILE).expect("Should open image");
        img.save(SAVED_FILE).ok();
        update_frame(img.width(), img.height(), SAVED_FILE);
    });
    but_custom_kernel.set_callback(move |_| {
        let custom_kernel = [
            [
                fetch_input_val(&kernel_0),
                fetch_input_val(&kernel_1),
                fetch_input_val(&kernel_2),
            ],
            [
                fetch_input_val(&kernel_3),
                fetch_input_val(&kernel_4),
                fetch_input_val(&kernel_5),
            ],
            [
                fetch_input_val(&kernel_6),
                fetch_input_val(&kernel_7),
                fetch_input_val(&kernel_8),
            ],
        ];
        apply_kernel_to_image(custom_kernel, false, true);
    });

    but_rotate_left.set_callback(move |_| {
        apply_function_to_image(&image_ops::point_ops::rotate_90_degrees_left);
    });

    but_rotate_right.set_callback(move |_| {
        apply_function_to_image(&image_ops::point_ops::rotate_90_degrees_right);
    });

    but_zoom_in.set_callback(move |_| {
        apply_function_to_image(&image_ops::matrix_ops::zoom_in);
    });

    but_zoom_out.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::matrix_ops::zoom_out(&img, fetch_input_val(&out_sx), fetch_input_val(&out_sy))
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width(), img.height(), SAVED_FILE);
    });

    but_histogram_matching.set_callback(|_| {
        let mut file_chooser = fetch_file();
        let path = file_chooser.value(0).expect("Should have choosen file");
        let image_to_match = image::open(
            file_chooser
                .value(0)
                .expect("Should have choosen file")
                .clone(),
        )
        .expect("Should open image")
        .into_rgb8();
        let base_image = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        match_histogram(&base_image, &image_to_match)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(base_image.width(), base_image.height(), SAVED_FILE);
        update_frame(image_to_match.width(), image_to_match.height(), &path);
    });

    window.show();
    app.run().ok();
}

fn normalize_gain(a: &mut f32) {
    *a = if *a <= 0.0 {
        0.1
    } else {
        if *a > 255.0 {
            255.0
        } else {
            *a
        }
    };
}

fn normalize_bias(b: &mut f32) {
    *b = if *b > 255.0 {
        255.0
    } else if *b < -255.0 {
        -255.0
    } else {
        *b
    };
}

fn apply_function_to_image(
    func: &dyn Fn(
        &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) {
    let img = image::open(SAVED_FILE)
        .expect("Should open image")
        .into_rgb8();
    func(&img).save(SAVED_FILE).expect("Should save image");
    update_frame(img.width(), img.height(), SAVED_FILE);
}

fn calc_window_height(height: u32) -> i32 {
    (height + 150).max(750) as i32
}

fn calc_window_width(width: u32) -> i32 {
    (width + 100).max(400) as i32
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
    image_ops::matrix_ops::apply_conv(kernel, &image, should_clamp)
        .save(SAVED_FILE)
        .expect("Should save image");
    update_frame(image.width(), image.height(), SAVED_FILE);
}

fn update_frame(width: u32, height: u32, file_path: &str) {
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

fn fetch_input_val(input: &Input) -> f32 {
    input.value().trim().parse().expect("Should have number!")
}
