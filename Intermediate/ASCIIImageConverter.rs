#[cfg(feature = "intermediate")]
mod intermediate {
    use image::{io::Reader as ImageReader, GrayImage};
    use std::fs::File;
    use std::io::{self, Write};

    #[derive(Debug)]
    pub enum AsciiError {
        ImageLoad(image::ImageError),
        FileCreation(io::Error),
        FileWrite(io::Error),
    }

    impl From<image::ImageError> for AsciiError {
        fn from(err: image::ImageError) -> Self {
            AsciiError::ImageLoad(err)
        }
    }

    impl From<io::Error> for AsciiError {
        fn from(err: io::Error) -> Self {
            AsciiError::FileCreation(err)
        }
    }

    pub struct AsciiConverter {
        chars: &'static str,
        h_step: u32,
        v_step: u32,
    }

    impl AsciiConverter {
        pub fn new() -> Self {
            Self {
                chars: "@#*+=:-. ",
                h_step: 4,
                v_step: 8,
            }
        }

        pub fn load_image(&self, path: &str) -> Result<GrayImage, AsciiError> {
            let img = ImageReader::open(path)?.decode()?;
            Ok(img.to_luma8())
        }

        pub fn convert_to_ascii(&self, gray_img: &GrayImage) -> String {
            let (width, height) = gray_img.dimensions();
            let mut ascii_art = String::new();

            for y in (0..height).step_by(self.v_step as usize) {
                for x in (0..width).step_by(self.h_step as usize) {
                    let pixel = gray_img.get_pixel(x, y);
                    let brightness = pixel[0] as usize;
                    let char_index = brightness * (self.chars.len() - 1) / 255;
                    let ascii_char = self.chars.chars().nth(char_index).unwrap();
                    ascii_art.push(ascii_char);
                }
                ascii_art.push('\n');
            }

            ascii_art
        }

        pub fn save_to_file(&self, content: &str, filename: &str) -> Result<(), AsciiError> {
            let mut file = File::create(filename).map_err(AsciiError::FileCreation)?;
            file.write_all(content.as_bytes()).map_err(AsciiError::FileWrite)?;
            Ok(())
        }
    }

    pub fn run() -> Result<(), AsciiError> {
        println!("=== INTERMEDIATE VERSION ===");

        let converter = AsciiConverter::new();

        let gray_img = converter.load_image("input.jpg")?;
        let ascii_art = converter.convert_to_ascii(&gray_img);

        println!("{}", ascii_art);

        converter.save_to_file(&ascii_art, "output.txt")?;
        println!("Saved ASCII art to output.txt");

        Ok(())
    }
}

fn main() {
    #[cfg(feature = "intermediate")]
    intermediate::run();
}
