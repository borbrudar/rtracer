

use super::rvec3::*;
pub type Color = Rvec3;

pub fn write_color(pixel_color : &mut Color){
    println!("{} {} {}", (255.999 * pixel_color.x()) as i32, (255.999 * pixel_color.y()) as i32, (255.999 * pixel_color.z()) as i32);
}
