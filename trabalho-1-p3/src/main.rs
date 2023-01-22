use opencv::{highgui, imgproc::gaussian_blur, prelude::*, videoio, Result};

fn main() -> Result<()> {
    let mut cam =
        videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Should be able to open camera!");

    let mut frame = Mat::default();

    let window = highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    let mut kernel_size = 1;
    highgui::create_trackbar(
        "Gaussian",
        "window",
        Option::Some(&mut kernel_size),
        255,
        None,
    )?;

    loop {
        cam.read(&mut frame)
            .expect("Should be able to acquire new frame!");

        let mut frame_out = Mat::default();

        apply_gaussian(&frame, &mut frame_out, kernel_size)?;

        highgui::imshow("window", &frame_out)?;

        let key = highgui::wait_key(1)?;
        if key == 113 || key == 27 {
            break;
        }
    }

    cam.release()?;
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
        opencv::core::Size_ {
            width: copied_kernel,
            height: copied_kernel,
        },
        0.0,
        0.0,
        0,
    )?;
    Ok(())
}
