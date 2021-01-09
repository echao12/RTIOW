use std::io;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    //structuring .ppm format. P3 means colors in ASCII, then col x row, then max value.
    let mut file = File::create("image.ppm").expect("failed to create file");
    let line: String = format!("P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", line);
    write!(file, "{}", line);//this macro will format and return a string
    //note, rows written top->bottom and pixels from left -> right
    for j in (0..IMAGE_HEIGHT).rev() {
        //println!("height:{}", i);
        for i in 0..IMAGE_WIDTH {
            //top left starts as full green and will travel -> to red in a diagonal.
            let r = (i as f64) / ((IMAGE_WIDTH-1) as f64);//converting to range of 0->1 using denominator to be width or height
            let g = (j as f64) / ((IMAGE_HEIGHT-1) as f64);
            let b: f64 = 0.25;

            //generate pixel position's RGB
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            write!(file,"{} {} {}\n", ir, ig, ib);//cant use '?' in functions that return () values
        }
    }
    println!("Finished writing to .ppm file...");
}
