pub mod camera;
pub mod color;
pub mod hit;
pub mod hitlist;
pub mod interval;
pub mod material;
pub mod ray;
pub mod rvec3;
pub mod sphere;
pub mod utility;

use camera::*;
use color::*;
use hit::*;
use hitlist::*;
use interval::*;
use material::*;
use ray::*;
use rvec3::*;
use sphere::*;
use std::rc::Rc;
use utility::*;

pub fn main() {
    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 10;
    cam.max_depth = 10;

    //world
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new_arg(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new_arg(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new_arg(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new_arg(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(
        Point3::new_arg(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new_arg(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new_arg(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new_arg(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    cam.render(&mut world);
}
