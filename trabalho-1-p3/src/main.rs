use opencv::{
    core::{
        Size_, 
    },
    highgui::{self, ButtonCallback, QtButtonTypes, QT_PUSH_BUTTON},
    imgproc::{},
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

    let width = cam.get(3)? as i32;
    let height = cam.get(4)? as i32;

    let mut video_writer = VideoWriter::new(
        "target/video.avi",
        VideoWriter::fourcc('M', 'J', 'P', 'G')?,
        30.0,
        Size_::new(width, height),
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

mod video_ops;
