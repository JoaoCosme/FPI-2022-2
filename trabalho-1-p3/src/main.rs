use opencv::{
    core::Size_,
    highgui::{self},
    prelude::*,
    videoio::{self, VideoWriter},
    Result,
};

mod actions_enum;
mod video_ops;

fn main() -> Result<()> {
    let mut cam =
        videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Should be able to open camera!");

    let mut frame = Mat::default();

    let mut is_recording = false;

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
    let add_bright = 10.0;

    loop {
        cam.read(&mut frame)
            .expect("Should be able to acquire new frame!");

        is_recording = true;

        let mut frame_out = frame.clone();

        // video_ops::apply_canny(&frame_out.clone(), &mut frame_out)?;
        // video_ops::apply_sobel(&frame_out.clone(), &mut frame_out)?;

        video_ops::apply_gaussian(&frame_out.clone(), &mut frame_out, kernel_size)?;
        video_ops::apply_negative(&frame_out.clone(), &mut frame_out)?;
        video_ops::apply_bright_up(&frame_out.clone(), &mut frame_out, add_bright)?;
        video_ops::apply_conversion_to_gray(&frame_out.clone(), &mut frame_out)?;
        video_ops::apply_rotation(&frame_out.clone(), &mut frame_out)?;
        video_ops::apply_mirror(&frame_out.clone(), &mut frame_out)?;

        // // let apply_contrast = 2.0;
        // // frame.convert_to(&mut frame_out, CV_8U, apply_contrast, 0.0)?;

        if !is_recording {
            video_ops::apply_resize(&frame_out.clone(), &mut frame_out)?;
        } else {
            video_writer.write(&frame_out)?;
        }

        highgui::imshow("window", &frame_out)?;
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
