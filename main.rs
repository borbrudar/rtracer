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
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod perlin;


use hit::Hittable;
use image::GenericImageView;
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
use crate::bvh::*;
use crate::texture::*;

pub fn random_spheres(){
    let mut cam = Camera::new();

    //cam.image_width = 1600;
    //cam.samples_per_pixel = 70;
    //cam.max_depth = 500;
    cam.image_width  = 400;
    cam.samples_per_pixel =10;
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

    //let checker = Rc::new(CheckerTexture::new_color(0.32, Color::new_arg(0.2, 0.3, 0.1),Color::new_arg(0.9, 0.9, 0.9)));
    //world.add(Rc::new(Sphere::new(Point3::new_arg(0.0,-1000.0,0.0), 1000.0, Rc::new(RefCell::new(Lambertian::new_ptr(checker))) )));
    
    let ground_material = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.5,0.5,0.5))));
    world.add(Rc::new(Sphere::new(Point3::new_arg(0.0,-1000.0,0.0), 1000.0, ground_material )));

    /*
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
                    world.add(Rc::new(Sphere::new_movable(center,center2,0.2,sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_vec_range(0.5,1.0);
                    let fuzz = random_range(0.0, 0.5);
                    sphere_material = Rc::new(RefCell::new(Metal::new(albedo,fuzz)));
                    world.add(Rc::new(Sphere::new(center,0.2,sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(RefCell::new(Dielectric::new(1.5)));
                    world.add(Rc::new(Sphere::new(center,0.2,sphere_material)));
                }
            }
        }
    }
    */
    let material_1 = Rc::new(RefCell::new(Dielectric::new(1.5)));
    world.add(Rc::new(Sphere::new(Point3::new_arg( 0.0,1.0,0.0), 1.0, material_1)));

    let material_2 = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.4,0.2,0.1))));
    world.add(Rc::new(Sphere::new(Point3::new_arg( -4.0,   1.0,1.0), 1.0, material_2)));

    let material_3 = Rc::new(RefCell::new(Metal::new(Color::new_arg(0.7,0.6,0.5),0.0)));
    world.add(Rc::new(Sphere::new(Point3::new_arg(4.0,  1.0, 0.0), 1.0, material_3)));

    // ?????
    let node = BvhNode::new_list(world);
    let mut vc : Vec<Rc<dyn Hittable>> = Vec::new();
    vc.push(Rc::new(node));
    world = HittableList::new_arg(vc);

    cam.render(&mut world);
}

pub fn two_spheres(){
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::new_color(0.32, Color::new_arg(0.2, 0.3, 0.1),Color::new_arg(0.9, 0.9, 0.9)));

    world.add(Rc::new(Sphere::new(Point3::new_arg(0.0,-10.0,0.0), 10.0, Rc::new(RefCell::new(Lambertian::new_ptr(checker.clone()))) )));
    world.add(Rc::new(Sphere::new(Point3::new_arg(0.0, 10.0,0.0), 10.0, Rc::new(RefCell::new(Lambertian::new_ptr(checker))) )));

    let mut cam = Camera::new();

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;

    cam.vfov     = 20.0;
    cam.lookfrom = Point3::new_arg(13.0,2.0,3.0);
    cam.lookat   = Point3::new();
    cam.vup      = Rvec3::new_arg(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&mut world);
    
}


pub fn earth() {
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg".to_string()));
    let earth_surface = Rc::new(RefCell::new(Lambertian::new_ptr(earth_texture)));
    let globe = Rc::new(Sphere::new(Point3::new(), 2.0, earth_surface));

    let mut cam = Camera::new();
    let mut world : Vec<Rc<dyn Hittable>> =  Vec::new();
    world.push(globe);

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 1200;
    cam.samples_per_pixel = 1000;
    cam.max_depth         = 50;

    cam.vfov     = 20.0;
    cam.lookfrom = Point3::new_arg(0.0,0.0,12.0);
    cam.lookat   = Point3::new();
    cam.vup      = Rvec3::new_arg(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&mut HittableList::new_arg(world));
}

pub fn two_perlin_spheres() {
    let mut world = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new());
    world.add(Rc::new(Sphere::new(Point3::new_arg(0.0,-1000.0,0.0),1000.0, Rc::new(RefCell::new(Lambertian::new_ptr(pertext.clone()))))));
    world.add(Rc::new(Sphere::new(Point3::new_arg(0.0,    2.0,0.0),   2.0, Rc::new(RefCell::new(Lambertian::new_ptr(pertext))))));

    let mut cam = Camera::new();

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;

    cam.vfov     = 20.0;
    cam.lookfrom = Point3::new_arg(13.0,2.0,3.0);
    cam.lookat   = Point3::new();
    cam.vup      = Rvec3::new_arg(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&mut world);
}

pub fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    match 4 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        _ => ()
    }
}
