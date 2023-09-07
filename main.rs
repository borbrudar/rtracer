pub mod rvec3;
pub mod color;
pub mod ray;
pub mod hit;
pub mod sphere;
pub mod hitlist;
pub mod utility;
pub mod interval;
pub mod camera;

use rvec3::*;
use color::*;
use ray::*;
use hit::*;
use hitlist::*;
use sphere::*;
use interval::*;
use utility::*;
use camera::*;

pub fn main(){
    let mut cam = Camera::new();
    
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width  = 400;
    cam.samples_per_pixel = 100;

    //world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new_arg(0.0,0.0,-1.0),0.5)));
    world.add(Box::new(Sphere::new(Point3::new_arg(0.0,-100.5,-1.0),100.0)));

    cam.render(&mut world);
}
