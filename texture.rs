use crate::interval::Interval;
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
    fn value(&self, _u : f64, _v : f64, _p : Point3) -> Color{
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
            self.even.value(u,v,p)
        }else {
            self.odd.value(u,v,p)
        }
    }
    
}



use image::DynamicImage;
use image::GenericImageView;
use image::io::Reader as ImageReader;



pub struct ImageTexture{
    img : DynamicImage,
}

impl ImageTexture{
    pub fn new(filename : String) -> Self{
        Self { 
            img : ImageReader::open(filename).unwrap().decode().unwrap()
        }
    }

    pub fn clamp(x : i32, low : i32 , high : i32) -> i32{
        // Return the value clamped to the range [low, high).
        if x < low  {return low;}
        if x < high {return x;}
        high-1
    }

    pub fn pixel_data(&self, mut x : i32, mut y : i32) -> Color{
        // Return the address of the three bytes of the pixel at x,y (or magenta if no data) -- which doesnt happen in rust?

        x = ImageTexture::clamp(x,0,self.img.width() as i32);
        y = ImageTexture::clamp(y,0,self.img.height() as i32);

        let pixel = self.img.get_pixel(x as u32, y as u32);
        Color::new_arg(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
    }

}

 
impl Texture for ImageTexture {
    fn value(&self, mut u : f64, mut v : f64, _p : Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.img.height() == 0 {
            return Color::new_arg(0.0, 1.0, 1.0)
        }
        
        // Clamp input texture coordinates to [0,1] x [1,0]
        u = Interval::new_arg(0.0, 1.0).clamp(u);
        v = 1.0 - Interval::new_arg(0.0,1.0).clamp(v); // Flip V to image coordinates


        let i = (u * (self.img.width() as f64) ) as i32;
        let j = (v * (self.img.height() as f64)) as i32;
        let pixel = self.pixel_data(i,j);
        
        let color_scale = 1.0 / 255.0;
        Color::new_arg(color_scale * pixel[0], color_scale* pixel[1], color_scale * pixel[2])
    }

}