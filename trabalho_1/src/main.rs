mod image_ops;
use image::{GenericImageView};
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
        image_ops::horizontal_flip(&img)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32);
    });
    but_gray.set_callback(move |_| {
        let img = image::open(SAVED_FILE).expect("Should open image").into_rgb8();
        image_ops::make_gray_image(&img)
        .save(SAVED_FILE)
        .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32);
    });
    but_vertical.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::vertical_flip(&img)
        .save(SAVED_FILE)
        .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32);
    });
    but_equalize.set_callback(move |_| {
        let img = image::open(SAVED_FILE).expect("Should open image").into_rgb8();
        image_ops::equalize_image(
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

