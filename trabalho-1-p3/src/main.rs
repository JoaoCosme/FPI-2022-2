use opencv::{highgui, prelude::*, videoio, Result};

fn main() -> Result<()> {
    let mut cam =
        videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Should be able to open camera!");

    let mut frame = Mat::default();

    loop {
        cam.read(&mut frame)
            .expect("Should be able to acquire new frame!");

        highgui::imshow("window", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 || key == 27 {
            break;
        }
    }

    cam.release()?;
    Ok(())
}
