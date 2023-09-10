use crate::rvec3::*;
use crate::color::*;

pub trait Texture{
    fn value(&self, u : f64, v : f64, p : Point3) -> Color;
}

pub struct SolidColor{
    color_value : Color,
}

impl SolidColor{
    pub fn new(c : Color) -> Self {
        Self { 
            color_value : c,
        }
    }
    pub fn new_rgb(r : f64, g : f64, b : f64) -> Self {
        Self {
            color_value : Color::new_arg(r,g,b),
        }
    }
}

impl Texture for SolidColor{
    fn value(&self, u : f64, v : f64, p : Point3) -> Color{
        self.color_value
    }

}
