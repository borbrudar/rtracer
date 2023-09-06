

use super::rvec3::*;
type color = Rvec3;

pub fn write_color(pixel_color : &mut color){
    println!("{} {} {}", (255.999 * pixel_color.x()) as i32, (255.999 * pixel_color.y()) as i32, (255.999 * pixel_color.z()) as i32);
}
