use opencv::{
    self,
    core::{
        self, add_weighted, convert_scale_abs, Size_, BORDER_DEFAULT, CV_16S, CV_8U,
        ROTATE_90_CLOCKWISE,
    },
    imgproc::{canny, cvt_color, gaussian_blur, resize, sobel, COLOR_BGR2GRAY, INTER_AREA},
    prelude::{Mat, MatTraitConst},
    Result,
};

pub(crate) fn apply_resize_down(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    resize(frame, frame_out, Size_::new(0, 0), 0.5, 0.5, INTER_AREA)?;
    Ok(())
}

pub(crate) fn apply_mirror_horizontal(
    frame: &Mat,
    frame_out: &mut Mat,
) -> Result<(), opencv::Error> {
    opencv::core::flip(frame, frame_out, 1)?;
    Ok(())
}

pub(crate) fn apply_mirror_vertical(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    opencv::core::flip(frame, frame_out, 0)?;
    Ok(())
}

pub(crate) fn apply_rotation(
    frame_out: &mut Mat,
    number_of_rots: i32,
) -> Result<(), opencv::Error> {
    (0..number_of_rots % 4).for_each(|_| {
        opencv::core::rotate(&frame_out.clone(), frame_out, ROTATE_90_CLOCKWISE).ok();
    });
    Ok(())
}

pub(crate) fn apply_conversion_to_gray(
    frame: &Mat,
    frame_out: &mut Mat,
) -> Result<(), opencv::Error> {
    cvt_color(frame, frame_out, COLOR_BGR2GRAY, 0)?;
    Ok(())
}

pub(crate) fn apply_bright_adjustment(
    frame: &Mat,
    frame_out: &mut Mat,
    add_bright: f64,
) -> Result<(), opencv::Error> {
    frame.convert_to(frame_out, CV_8U, 1.0, add_bright)?;
    Ok(())
}

pub(crate) fn apply_contrast(
    frame: &Mat,
    frame_out: &mut Mat,
    alpha: f64,
) -> Result<(), opencv::Error> {
    frame.convert_to(frame_out, CV_8U, alpha, 0.0)?;
    Ok(())
}

pub(crate) fn apply_negative(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    frame.convert_to(frame_out, CV_8U, -1.0, 255.0)?;
    Ok(())
}

pub(crate) fn apply_sobel(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    let mut grad_x = Mat::default();
    let mut grad_y = Mat::default();
    let mut abs_grad_x = Mat::default();
    let mut abs_grad_y = Mat::default();
    let mut gray_image = frame_out.clone();

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

pub(crate) fn apply_canny(frame: &Mat, frame_out: &mut Mat) -> Result<(), opencv::Error> {
    pre_processing_gaussian(frame, frame_out)?;
    canny(&frame_out.clone(), frame_out, 40.0, 100.0, 3, false)?;
    Ok(())
}

pub(crate) fn pre_processing_gaussian(
    frame: &Mat,
    frame_out: &mut Mat,
) -> Result<(), opencv::Error> {
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

pub(crate) fn apply_gaussian(
    frame: &Mat,
    frame_out: &mut Mat,
    kernel_size: i32,
) -> Result<(), opencv::Error> {
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
