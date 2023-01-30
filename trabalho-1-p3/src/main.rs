use opencv::{
    core::{
        add_weighted, convert_scale_abs, Size_, BORDER_DEFAULT, CV_16S, CV_8U, ROTATE_90_CLOCKWISE,
    },
    highgui::{self, ButtonCallback, QtButtonTypes, QT_PUSH_BUTTON},
    imgproc::{canny, cvt_color, gaussian_blur, resize, sobel, COLOR_BGR2GRAY, INTER_AREA},
    prelude::*,
    videoio::{self, VideoWriter, CAP_FFMPEG},
    Result,
};

fn main() -> Result<()> {
    let mut cam =
        videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Should be able to open camera!");

    let mut frame = Mat::default();

    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    let mut kernel_size = 1;
    highgui::create_trackbar(
        "Gaussian",
        "window",
        Option::Some(&mut kernel_size),
        255,
        None,
    )?;

    // highgui::create_button("Fuzzy", None, 0, false)?;

    let mut video_writer = VideoWriter::new(
        "target/video.avi",
        VideoWriter::fourcc('M', 'J', 'P', 'G')?,
        30.0,
        Size_::new(cam.get(3)? as i32, cam.get(4)? as i32),
        true,
    )?;

    println!("Recording started");

    loop {
        cam.read(&mut frame)
            .expect("Should be able to acquire new frame!");

        let mut frame_out = Mat::default();

        // apply_gaussian(&frame, &mut frame_out, kernel_size)?;
        // apply_canny(&frame, &mut frame_out)?;
        // apply_sobel(&frame, &mut frame_out)?;
        // apply_negative(&frame, &mut frame_out)?;
        // let add_bright = 10.0;
        // apply_bright_up(&frame, &mut frame_out, add_bright)?;

        // let apply_contrast = 2.0;
        // frame.convert_to(&mut frame_out, CV_8U, apply_contrast, 0.0)?;

        // apply_conversion_to_gray(&frame, &mut frame_out)?;

        // apply_rotation(&frame, &mut frame_out)?;

        // apply_mirror(&frame, &mut frame_out)?;

        // apply_resize(&frame, &mut frame_out)?;

        video_writer.write(&frame)?;

        // highgui::imshow("window", &frame_out)?;
        highgui::imshow("original", &frame)?;

        let key = highgui::wait_key(1)?;
        if key == 113 || key == 27 {
            break;
        }
    }

    video_writer.release()?;
    cam.release()?;
    Ok(())
}

fn apply_resize(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    resize(frame, frame_out, Size_::new(0, 0), 0.5, 0.5, INTER_AREA)?;
    Ok(())
}

fn apply_mirror(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    opencv::core::flip(frame, frame_out, 1)?;
    Ok(())
}

fn apply_rotation(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    opencv::core::rotate(frame, frame_out, ROTATE_90_CLOCKWISE)?;
    Ok(())
}

fn apply_conversion_to_gray(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    cvt_color(frame, frame_out, COLOR_BGR2GRAY, 0)?;
    Ok(())
}

fn apply_bright_up(frame: &Mat, frame_out: &mut Mat, add_bright: f64) -> Result<(), opencv::Error> {
    frame.convert_to(frame_out, CV_8U, 1.0, add_bright)?;
    Ok(())
}

fn apply_negative(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    frame.convert_to(frame_out, CV_8U, -1.0, 255.0)?;
    Ok(())
}

fn apply_sobel(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    let mut grad_x = Mat::default();
    let mut grad_y = Mat::default();
    let mut abs_grad_x = Mat::default();
    let mut abs_grad_y = Mat::default();
    let mut gray_image = Mat::default();

    pre_processing_gaussian(frame, frame_out)?;

    cvt_color(frame_out, &mut gray_image, COLOR_BGR2GRAY, 0)?;

    sobel(
        &gray_image,
        &mut grad_x,
        CV_16S,
        1,
        0,
        3,
        1.0,
        0.0,
        BORDER_DEFAULT,
    )?;
    sobel(
        &gray_image,
        &mut grad_y,
        CV_16S,
        0,
        1,
        3,
        1.0,
        0.0,
        BORDER_DEFAULT,
    )?;

    convert_scale_abs(&grad_x, &mut abs_grad_x, 1.0, 0.0)?;
    convert_scale_abs(&grad_y, &mut abs_grad_y, 1.0, 0.0)?;

    add_weighted(&abs_grad_x, 0.5, &abs_grad_y, 0.05, 0.0, frame_out, -1)?;

    Ok(())
}

fn apply_canny(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    pre_processing_gaussian(frame, frame_out)?;
    canny(&frame_out.clone(), frame_out, 40.0, 100.0, 3, false)?;
    Ok(())
}

fn pre_processing_gaussian(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    gaussian_blur(
        frame,
        frame_out,
        Size_ {
            width: 5,
            height: 5,
        },
        0.0,
        0.0,
        0,
    )?;
    Ok(())
}

fn apply_gaussian(frame: &Mat, frame_out: &mut Mat, kernel_size: i32) -> Result<(), opencv::Error> {
    let mut copied_kernel = kernel_size;

    if copied_kernel % 2 == 0 {
        copied_kernel += 1;
    }

    gaussian_blur(
        frame,
        frame_out,
        Size_ {
            width: copied_kernel,
            height: copied_kernel,
        },
        0.0,
        0.0,
        0,
    )?;
    Ok(())
}
