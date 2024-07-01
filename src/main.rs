use image::{self};
use image::GenericImageView;
use std::env;
use std::fs::File;
use std::io::Write;

const  DENSITY : &str = "Ñ@#W$9876543210?!abc;:+=-,._";
fn main() {
    let density_vec : Vec<char> = DENSITY.chars().collect();
    let args : Vec<String> = env::args().collect();

    if args.len() < 2{
        panic!("\nError: provide an image path\n");
    }
    let img = image::open(&args[1]).expect("No image was found");
    let mut ascii_buf = String::new();
    let mut prev : u32 = 0;
    for pixel in img.pixels(){
        if prev != pixel.1{
            prev = pixel.1;
            ascii_buf.push('\n');
            print!("\n");
        }
        let rgba = pixel.2;
        let mut average : f32 = 0.0;
        average += rgba[0] as f32;
        average += rgba[1] as f32;
        average += rgba[2] as f32;
        average /=  3.0;
        let ind = (average/ 255.0 * 27.0) as usize;
        print!("{}",density_vec[ind]);
        ascii_buf.push(density_vec[ind]);
    }
    let mut file = File::create("ascii-output.txt").expect("Couldn't create a file");
    file.write_all(ascii_buf.as_bytes()).expect("Couldn't write bytes to file");
    println!("\n\nOutput was written to 'ascii-output.txt'\n");

}
