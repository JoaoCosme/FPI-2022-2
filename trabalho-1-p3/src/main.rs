use opencv::{
    core::Size_,
    highgui::{self, ButtonCallback, QtButtonTypes, QT_PUSH_BUTTON},
    imgproc::{canny, gaussian_blur},
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

        apply_canny(&frame, &mut frame_out)?;

        highgui::imshow("window", &frame_out)?;

        let key = highgui::wait_key(1)?;
        if key == 113 || key == 27 {
            break;
        }
    }

    cam.release()?;
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
