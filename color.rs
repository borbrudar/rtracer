use super::rvec3::*;
use crate::interval::*;
pub type Color = Rvec3;

pub fn write_color(pixel_color : &mut Color, samples_per_pixel : i32){
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    let mut intensity = Interval::new_arg(0.0,0.999);
    println!("{} {} {}", (256.0 * intensity.clamp(r)) as i32, (256.0 * intensity.clamp(g)) as i32, (256.0 * intensity.clamp(b)) as i32);
}
