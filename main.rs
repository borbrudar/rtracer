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

use hitlist::*;
use sphere::*;


use camera::*;
use material::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::env;

pub fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    //cam.image_width = 1600;
    //cam.samples_per_pixel = 70;
    //cam.max_depth = 500;
    cam.image_width  = 400;
    cam.samples_per_pixel = 20;
    cam.max_depth = 30;
    cam.vfov = 20.0;

    cam.lookfrom = Point3::new_arg(-2.0,2.0,1.0);
    cam.lookat   = Point3::new_arg(0.0,0.0,-1.0);
    cam.vup      = Point3::new_arg(0.0,1.0,0.0);

    //world
    let mut world = HittableList::new();

    let material_ground = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.8,0.8,0.0))));
    let material_center = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.1,0.2,0.5))));
    let material_left = Rc::new(RefCell::new(Dielectric::new(1.5)));
    let material_right = Rc::new(RefCell::new(Metal::new(Color::new_arg(0.8,0.6,0.2),0.0)));

    world.add(Box::new(Sphere::new(Point3::new_arg( 0.0,-100.5,-1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new_arg( 0.0,   0.0,-1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new_arg(-1.0,   0.0,-1.0), 0.5, material_left.clone())));
    world.add(Box::new(Sphere::new(Point3::new_arg(-1.0,   0.0,-1.0), -0.4, material_left)));
    world.add(Box::new(Sphere::new(Point3::new_arg( 1.0,   0.0,-1.0), 0.5, material_right)));

    cam.render(&mut world);
}
