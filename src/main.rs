use image::{DynamicImage, ImageBuffer, Luma};
use std::path::Path;

fn convert_to_bw<P: AsRef<Path>>(input_path: P, output_path: P) -> Result<(), image::ImageError> {
    // Read the image
    let img: DynamicImage = image::open(&input_path)?;
    
    // Convert to grayscale
    let gray_img = img.into_luma8();  // Changed from to_luma8() to into_luma8()
    
    // Convert to black and white with a threshold
    let threshold = 128u8;
    let bw_img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_fn(
        gray_img.width(),
        gray_img.height(),
        |x, y| {
            let pixel = gray_img.get_pixel(x, y);
            if pixel[0] > threshold {
                Luma([255u8]) // White
            } else {
                Luma([0u8])   // Black
            }
        }
    );
    
    // Save the result
    bw_img.save(output_path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match convert_to_bw("./image.png", "output.jpg") {
        Ok(_) => println!("Conversion complete! Check output.jpg"),
        Err(e) => println!("Error converting image: {}", e),
    }
    Ok(())
}