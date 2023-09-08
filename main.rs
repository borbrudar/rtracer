pub mod rvec3;
pub mod color;
pub mod ray;
pub mod hit;
pub mod sphere;
pub mod hitlist;
pub mod utility;
pub mod interval;
pub mod camera;
pub mod material;

use rvec3::*;
use color::*;
use ray::*;
use hit::*;
use hitlist::*;
use sphere::*;
use interval::*;
use utility::*;
use camera::*;
use material::*;
use std::rc::Rc;
use std::cell::RefCell;

pub fn main(){
    let mut cam = Camera::new();
    
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width  = 800;
    cam.samples_per_pixel = 20;
    cam.max_depth = 100;

    //world
    let mut world = HittableList::new();

    let material_ground = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.8,0.8,0.0))));
    let material_center = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.7,0.3,0.3))));
    let material_left = Rc::new(RefCell::new(Metal::new(Color::new_arg(0.8,0.8,0.8),0.3)));
    let material_right = Rc::new(RefCell::new(Metal::new(Color::new_arg(0.8,0.6,0.2),1.0)));

    world.add(Box::new(Sphere::new(Point3::new_arg( 0.0,-100.5,-1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new_arg( 0.0,   0.0,-1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new_arg(-1.0,   0.0,-1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new_arg( 1.0,   0.0,-1.0), 0.5, material_right)));

    cam.render(&mut world);
}
