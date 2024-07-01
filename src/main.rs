use image::{self};
use image::GenericImageView;
use std::fs::File;
use std::io::Write;
use std::str::Bytes;

fn main() {
    let density : &str = "Ñ@#W$9876543210?!abc;:+=-,._";
    let density_vec : Vec<char> = density.chars().collect();
    println!("Hello, world!");
    let img = image::open("funny-cat-resize.jpg").expect("no image was found");
    //println!("{}", 255/8);
    let mut new_img = String::new();
    let mut prev : u32 = 0;
    for pixel in img.pixels(){
        if prev != pixel.1{
            new_img.push('\n');
        }
        //println!("{} - {}", pixel.0, pixel.1);
        let rgba = pixel.2;
        let mut average : f32 = 0.0;
        average += rgba[0] as f32;
        average += rgba[1] as f32;
        average += rgba[2] as f32;
        average += rgba[3] as f32;
        average /=  4.0;
        //let r = (255-0)/(28-0);
        //let y = (average) * r;
        let ind = ((255.0-average) as f32 / 255.0 * 27.0) as usize;
        new_img.push(density_vec[ind]);
        //print!("{}", density_vec[ind]);
    }
    let mut file = File::create("bi.txt").expect("couldn't create a file");
    file.write_all(new_img.as_bytes());

}
