use crate::rvec3::*;
use crate::color::*;
use std::rc::Rc;

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


pub struct CheckerTexture{
    inv_scale : f64,
    even : Rc<dyn Texture>,
    odd : Rc<dyn Texture>,
}


impl CheckerTexture{
    pub fn new(_scale : f64, _even : Rc<dyn Texture>, _odd : Rc<dyn Texture>) -> Self {
        Self { 
            inv_scale : 1.0 / _scale,
            even : _even,
            odd : _odd,
        }
    }

    pub fn new_color(_scale : f64, c1 : Color, c2 : Color) -> Self{
        Self { 
            inv_scale : 1.0 / _scale,
            even : Rc::new(SolidColor::new(c1)),
            odd : Rc::new(SolidColor::new(c2)),
        }     
    }
}


impl Texture for CheckerTexture {
    fn value(&self, u : f64, v : f64, mut p : Point3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            return self.even.value(u,v,p);
        }else {
            return self.odd.value(u,v,p);
        }
    }
    
}