use image::{GenericImageView, ImageBuffer, DynamicImage, Pixel, Rgba, Luma, Rgb};
const IMAGE_SIZE:usize = 1000;

fn main(){
  let img = image::open("./src/test_images/Gramado_22k.jpg")
  .expect("Should open image");
  
  let (w,h) = img.dimensions();
  let mut output = ImageBuffer::new(w,h);
  let mut gray_image = [[0;IMAGE_SIZE];IMAGE_SIZE];

  for (x,y,pixel) in img.pixels(){
    let pixels = pixel.to_rgb().0;
    let gray_value = to_grayscale(pixels);
    let x = x as usize;
    let y = y as usize;
    gray_image[x][y] = gray_value;
    output.put_pixel(x as u32, y as u32, Rgb{
      0:[gray_value,gray_value,gray_value]
    });
  }
  let hist = make_histogram(gray_image);
  output.save("./image.jpeg").expect("Should save image");
}

fn to_grayscale(pixels:[u8;3]) -> u8{
    let red = pixels[0] as f64;
    let green = pixels[1] as f64;
    let blue = pixels[2] as f64;

    let new_val = 0.299*red + 0.587*green + 0.114*blue;
    let new_val = new_val as u8;
    return new_val;
}

fn make_histogram(gray_image:[[u8;IMAGE_SIZE];IMAGE_SIZE]) -> [i32;256]{
  let mut histogram:[i32;256] = [0; 256];
  for x in 0..IMAGE_SIZE{
    for y in 0..IMAGE_SIZE{
      histogram[gray_image[x][y] as usize] +=1;
    }
  }
  return histogram;
}