use super::rvec3::*;
use crate::interval::*;
pub type Color = Rvec3;

pub fn linear_to_gamma(linear_component : f64) -> f64 {
    linear_component.sqrt()
}


pub fn calculate_true_color(pixel_color : &mut Color, samples_per_pixel : i32) -> Color{
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;


    // Apply the linear to gamma transform.
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);


    let mut intensity = Interval::new_arg(0.0,0.999);
    let true_color = Color {
        e: [ (256.0 * intensity.clamp(r)), (256.0 * intensity.clamp(g)), (256.0 * intensity.clamp(b))],
    };
    println!("{} {} {}", (256.0 * intensity.clamp(r)) as i32, (256.0 * intensity.clamp(g)) as i32, (256.0 * intensity.clamp(b)) as i32);
    true_color
}
