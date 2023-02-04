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

    // println!("Select an action:");
    let mut should_mirror = false;
    let mut should_neg = false;
    let mut should_gray = false;
    let mut should_resize = false;
    let mut should_canny = false;
    let mut should_sobel = false;
    let mut should_record = false;

    let flip_bool = |value: &mut bool| *value = !*value;
    let stop_start_record = |is_recording: &mut bool| {
        if *is_recording {
            println!("Stopping recording!")
        } else {
            println!("Start recording!")
        }
        flip_bool(is_recording);
    };
    let mut rotate = 0;
    let mut bright = 0.0;
    let mut contrast = 1.0;

    loop {
        let key = highgui::wait_key(1)?;
        let char_input = key as u8 as char;

        cam.read(&mut frame)
            .expect("Should be able to acquire new frame!");

        let mut frame_out = frame.clone();

        match char_input {
            'm' => flip_bool(&mut should_mirror),
            'n' => flip_bool(&mut should_neg),
            'g' => flip_bool(&mut should_gray),
            'r' => rotate += 1,
            '.' => bright += 10.0,
            ',' => bright -= 10.0,
            '=' => contrast += 0.1,
            '-' => contrast -= 0.1,
            'b' => flip_bool(&mut should_sobel),
            'c' => flip_bool(&mut should_canny),
            'z' => flip_bool(&mut should_resize),
            's' => stop_start_record(&mut should_record),
            _ => (),
        }

        video_ops::apply_bright_up(&frame_out.clone(), &mut frame_out, bright)?;
        video_ops::apply_contrast(&frame_out.clone(), &mut frame_out, contrast)?;
        video_ops::apply_gaussian(&frame_out.clone(), &mut frame_out, kernel_size)?;

        if should_mirror {
            video_ops::apply_mirror(&frame_out.clone(), &mut frame_out)?;
        }
        if should_neg {
            video_ops::apply_negative(&frame_out.clone(), &mut frame_out)?;
        }

        if should_gray && !(should_sobel || should_canny) {
            video_ops::apply_conversion_to_gray(&frame_out.clone(), &mut frame_out)?;
        }

        if should_sobel {
            if should_gray {
                should_gray = false;
            }
            video_ops::apply_sobel(&frame_out.clone(), &mut frame_out)?;
        }

        if should_canny {
            if should_gray {
                should_gray = false;
            }
            video_ops::apply_canny(&frame_out.clone(), &mut frame_out)?;
        }

        if should_record {
            if should_resize {
                flip_bool(&mut should_resize)
            }
            video_writer.write(&frame_out)?;
        }

        if should_resize {
            if !is_recording {
                video_ops::apply_resize_down(&frame_out.clone(), &mut frame_out)?;
            }
        }

        video_ops::apply_rotation(&mut frame_out, rotate)?;
        highgui::imshow("window", &frame_out)?;
        highgui::imshow("original", &frame)?;

        if key == 113 || key == 27 {
            break;
        }
    }

    println!("Recording saved!");
    video_writer.release()?;
    cam.release()?;
    Ok(())
}
