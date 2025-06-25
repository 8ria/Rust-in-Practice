#[cfg(feature = "pro")]
mod pro {
    use image::{io::Reader as ImageReader, GrayImage, Pixel};
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::Path;

    #[derive(Debug, thiserror::Error)]
    pub enum AsciiError {
        #[error("Failed to load image: {0}")]
        ImageLoad(#[from] image::ImageError),
        #[error("Failed to create file: {0}")]
        FileCreation(#[from] io::Error),
    }

    pub trait AsciiCharset {
        const CHARS: &'static str;
        const CHAR_COUNT: usize = Self::CHARS.len();

        fn brightness_to_char(brightness: u8) -> char {
            let index = (brightness as usize * (Self::CHAR_COUNT - 1)) / 255;
            Self::CHARS.chars().nth(index).unwrap()
        }
    }

    pub struct StandardCharset;
    impl AsciiCharset for StandardCharset {
        const CHARS: &'static str = "@#*+=:-. ";
    }

    pub struct AsciiConverter<C: AsciiCharset, const H_STEP: u32, const V_STEP: u32> {
        _charset: std::marker::PhantomData<C>,
    }

    impl<C: AsciiCharset, const H_STEP: u32, const V_STEP: u32> AsciiConverter<C, H_STEP, V_STEP> {
        pub const fn new() -> Self {
            Self {
                _charset: std::marker::PhantomData,
            }
        }

        pub fn load_and_convert<P: AsRef<Path>>(&self, path: P) -> Result<String, AsciiError> {
            let img = ImageReader::open(path)?.decode()?;
            let gray_img = img.to_luma8();
            Ok(self.convert_image(&gray_img))
        }

        pub fn convert_image(&self, gray_img: &GrayImage) -> String {
            let (width, height) = gray_img.dimensions();
            let capacity = ((width / H_STEP + 1) * (height / V_STEP + 1)) as usize;
            let mut ascii_art = String::with_capacity(capacity);

            (0..height)
                .step_by(V_STEP as usize)
                .for_each(|y| {
                    (0..width)
                        .step_by(H_STEP as usize)
                        .for_each(|x| {
                            let brightness = gray_img.get_pixel(x, y).channels()[0];
                            ascii_art.push(C::brightness_to_char(brightness));
                        });
                    ascii_art.push('\n');
                });

            ascii_art
        }

        pub fn save_ascii<P: AsRef<Path>>(&self, content: &str, path: P) -> Result<(), AsciiError> {
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        }
    }

    type StandardConverter = AsciiConverter<StandardCharset, 4, 8>;

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        println!("=== PRO VERSION ===");

        const CONVERTER: StandardConverter = StandardConverter::new();

        let ascii_art = CONVERTER.load_and_convert("input.jpg")?;

        print!("{}", ascii_art);

        CONVERTER.save_ascii("output.txt", &ascii_art)?;
        println!("Saved ASCII art to output.txt");

        Ok(())
    }
}

fn main() {
    #[cfg(feature = "pro")]
    pro::run();
}
