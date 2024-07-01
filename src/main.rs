use image::{self};
use image::GenericImageView;
use std::fs::File;
use std::io::Write;
use std::str::Bytes;

fn main() {
    let density : &str = "Ñ@#W$9876543210?!abc;:+=-,._";
    let density_vec : Vec<char> = density.chars().collect();
    let img = image::open("funny-cat-resize.jpg").expect("no image was found");
    let mut ascii_buf = String::new();
    let mut prev : u32 = 0;
    for pixel in img.pixels(){
        if prev != pixel.1{
            prev = pixel.1;
            ascii_buf.push('\n');
        }
        let rgba = pixel.2;
        let mut average : f32 = 0.0;
        average += rgba[0] as f32;
        average += rgba[1] as f32;
        average += rgba[2] as f32;
        average += rgba[3] as f32;
        average /=  4.0;
        let ind = (average/ 255.0 * 27.0) as usize;
        ascii_buf.push(density_vec[ind]);
    }
    let mut file = File::create("ascii-output.txt").expect("couldn't create a file");
    file.write_all(ascii_buf.as_bytes());

}
