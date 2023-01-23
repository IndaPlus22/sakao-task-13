use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    gen_test_image();
}

fn gen_test_image() -> std::io::Result<()> {
    let ix:u32 = 200;
    let iy:u32 = 100;

    let mut file = File::create("pic.ppm")?;
    file.write_all((String::from("P3\n") + &ix.to_string() + " " +  &iy.to_string() + " \n255\n").as_bytes())?;

    for j in (0..iy).rev() {
        for i in 0..ix {
            let r: f32 = i as f32 / ix as f32;
            let g: f32 = j as f32 / iy as f32;
            let b: f32 = 0.2;

            let ir: i32 = (255.00 * r) as i32;
            let ig: i32 = (255.00 * g) as i32;
            let ib: i32 = (255.00 * b) as i32;

            let picture_string = ir.to_string() + " " + &ig.to_string() + " " + &ib.to_string() + "\n";
            file.write_all(picture_string.as_bytes())?;
        }
    }
    Ok(())
}