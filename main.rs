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
use crate::utility::*;

use camera::*;
use material::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::env;

pub fn main(){
    //env::set_var("RUST_BACKTRACE", "1");
    let mut cam = Camera::new();

    //cam.image_width = 1600;
    //cam.samples_per_pixel = 70;
    //cam.max_depth = 500;
    cam.image_width  = 400;
    cam.samples_per_pixel =100;
    cam.max_depth = 10;
    
    cam.vfov = 20.0;
    cam.aspect_ratio = 16.0 / 9.0;
    
    cam.vfov     = 20.0;
    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    cam.lookfrom = Point3::new_arg(13.0,2.0,3.0);
    cam.lookat   = Point3::new_arg(0.0,0.0,0.0);
    cam.vup      = Point3::new_arg(0.0,1.0,0.0);
    
    //world
    let mut world = HittableList::new();

    let ground_material = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.5,0.5,0.5))));
    world.add(Box::new(Sphere::new(Point3::new_arg(0.0,-1000.0,0.0), 1000.0,ground_material)));

    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_double();
            let center = Point3::new_arg((a as f64) + 0.9 * random_double(), 0.2 , (b as f64) + 0.9 * random_double());

            if (center - Point3::new_arg(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material : Rc<RefCell<dyn Material>>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_vec() * Color::random_vec();
                    sphere_material = Rc::new(RefCell::new(Lambertian::new(albedo)));
                    let center2 = center + Rvec3::new_arg(0.0, random_range(0.0,0.5), 0.0);
                    world.add(Box::new(Sphere::new_movable(center,center2,0.2,sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_vec_range(0.5,1.0);
                    let fuzz = random_range(0.0, 0.5);
                    sphere_material = Rc::new(RefCell::new(Metal::new(albedo,fuzz)));
                    world.add(Box::new(Sphere::new(center,0.2,sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(RefCell::new(Dielectric::new(1.5)));
                    world.add(Box::new(Sphere::new(center,0.2,sphere_material)));
                }
            }
        }
    }

    let material_1 = Rc::new(RefCell::new(Dielectric::new(1.5)));
    world.add(Box::new(Sphere::new(Point3::new_arg( 0.0,1.0,0.0), 1.0, material_1)));

    let material_2 = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.4,0.2,0.1))));
    world.add(Box::new(Sphere::new(Point3::new_arg( -4.0,   1.0,1.0), 1.0, material_2)));

    let material_3 = Rc::new(RefCell::new(Metal::new(Color::new_arg(0.7,0.6,0.5),0.0)));
    world.add(Box::new(Sphere::new(Point3::new_arg(4.0,  1.0, 0.0), 1.0, material_3)));


    cam.render(&mut world);
}
