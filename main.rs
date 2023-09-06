


const IM_WIDTH : i32= 256;
const IM_HEIGHT : i32= 256;

use std::io;
pub mod rvec3;
pub mod color;
use rvec3::Rvec3;
use color::*;

pub fn main(){
    // Render
    println!("P3\n{} {}\n255", &IM_WIDTH,&IM_HEIGHT);


for j in 0..IM_HEIGHT {
        for i in 0..IM_WIDTH{
            eprintln!("\rScanlines remaining: {}", IM_HEIGHT-j);
            let mut pixel_color : Rvec3 = Rvec3{e : [i as f64 / ((IM_WIDTH-1) as f64), j as f64 / ((IM_HEIGHT-1) as f64),0.0]};
            
            write_color(&mut pixel_color);
        }
    }


}
