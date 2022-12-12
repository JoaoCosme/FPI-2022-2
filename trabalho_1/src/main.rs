use image::{GenericImageView, ImageBuffer, DynamicImage, Pixel, Rgba, Luma, Rgb};

fn main(){
  let img = image::open("./src/test_images/Gramado_22k.jpg")
  .expect("Should open image");
  
  let (w,h) = img.dimensions();
  let mut output = ImageBuffer::new(w,h);

  for (x,y,pixel) in img.pixels(){
    let pixels = pixel.to_rgb().0;
    let red = pixels[0] as f64;
    let green = pixels[1] as f64;
    let blue = pixels[2] as f64;

    let new_val = 0.299*red + 0.587*green + 0.114*blue;
    let new_val = new_val as u8;
    output.put_pixel(x, y, Rgb{
      0:[new_val,new_val,new_val]
    });
  }
  output.save("./image.jpeg").expect("Should save image");
}