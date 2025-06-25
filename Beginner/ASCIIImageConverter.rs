use image::io::Reader as ImageReader;
use std::fs::File;
use std::io::Write;

fn main() {
    let img = ImageReader::open("input.jpg")
        .expect("Can't find input.jpg - put an image file named 'input.jpg' in this folder")
        .decode()
        .expect("Can't read the image - make sure it's a valid image file");

    let gray_img = img.to_luma8();
    let (width, height) = gray_img.dimensions();

    let chars = "@#*+=:-. ";

    let mut ascii_art = String::new();

    for y in (0..height).step_by(8) {  
        for x in (0..width).step_by(4) {   
            let pixel = gray_img.get_pixel(x, y);
            let brightness = pixel[0] as usize;
            let char_index = brightness * (chars.len() - 1) / 255;
            let ascii_char = chars.chars().nth(char_index).unwrap();
            ascii_art.push(ascii_char);
        }
        ascii_art.push('\n');
    }

    println!("{}", ascii_art);

    let mut file = File::create("output.txt").expect("Can't create output file");
    file.write_all(ascii_art.as_bytes()).expect("Can't write to file");

    println!("Saved ASCII art to output.txt");
}
