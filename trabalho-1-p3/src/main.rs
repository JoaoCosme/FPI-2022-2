use opencv::{
    core::{add_weighted, convert_scale_abs, Size_, BORDER_DEFAULT, CV_16S},
    highgui::{self, ButtonCallback, QtButtonTypes, QT_PUSH_BUTTON},
    imgproc::{canny, cvt_color, gaussian_blur, sobel, COLOR_BGR2GRAY},
    prelude::*,
    videoio, Result,
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

    loop {
        cam.read(&mut frame)
            .expect("Should be able to acquire new frame!");

        let mut frame_out = Mat::default();

        // apply_gaussian(&frame, &mut frame_out, kernel_size)?;
        // apply_canny(&frame, &mut frame_out)?;

        apply_sobel(&frame, &mut frame_out)?;

        highgui::imshow("window", &frame_out)?;

        let key = highgui::wait_key(1)?;
        if key == 113 || key == 27 {
            break;
        }
    }

    cam.release()?;
    Ok(())
}

fn apply_sobel(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    let mut grad_x = Mat::default();
    let mut grad_y = Mat::default();
    let mut abs_grad_x = Mat::default();
    let mut abs_grad_y = Mat::default();
    let mut gray_image = Mat::default();

    cvt_color(frame, &mut gray_image, COLOR_BGR2GRAY, 0)?;

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
    canny(&frame_out.clone(), frame_out, 40.0, 100.0, 3, false)?;
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
