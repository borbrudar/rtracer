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
pub mod quad;


use hit::Hittable;

use hit::RotateY;
use hit::Translate;
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
use crate::bvh::*;
use crate::texture::*;
use crate::quad::*;

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

    cam.background  = Color::new_arg(0.70, 0.80, 1.00);
    
    //world
    let mut world = HittableList::new();

    //let checker = Rc::new(CheckerTexture::new_color(0.32, Color::new_arg(0.2, 0.3, 0.1),Color::new_arg(0.9, 0.9, 0.9)));
    //world.add(Rc::new(Sphere::new(Point3::new_arg(0.0,-1000.0,0.0), 1000.0, Rc::new(RefCell::new(Lambertian::new_ptr(checker))) )));
    
    let ground_material = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.5,0.5,0.5))));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg(0.0,-1000.0,0.0), 1000.0, ground_material ))));

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
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg( 0.0,1.0,0.0), 1.0, material_1))));

    let material_2 = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.4,0.2,0.1))));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg( -4.0,   1.0,1.0), 1.0, material_2))));

    let material_3 = Rc::new(RefCell::new(Metal::new(Color::new_arg(0.7,0.6,0.5),0.0)));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg(4.0,  1.0, 0.0), 1.0, material_3))));

    // ?????
    let node = BvhNode::new_list(world);
    let vc : Vec<Rc<RefCell<dyn Hittable>>> = vec![Rc::new(RefCell::new(node))];
    world = HittableList::new_arg(vc);

    cam.render(&mut world);
}

pub fn two_spheres(){
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::new_color(0.32, Color::new_arg(0.2, 0.3, 0.1),Color::new_arg(0.9, 0.9, 0.9)));

    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg(0.0,-10.0,0.0), 10.0, Rc::new(RefCell::new(Lambertian::new_ptr(checker.clone()))) ))));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg(0.0, 10.0,0.0), 10.0, Rc::new(RefCell::new(Lambertian::new_ptr(checker))) ))));

    let mut cam = Camera::new();

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;

    cam.vfov     = 20.0;
    cam.lookfrom = Point3::new_arg(13.0,2.0,3.0);
    cam.lookat   = Point3::new();
    cam.vup      = Rvec3::new_arg(0.0,1.0,0.0);

    cam.background  = Color::new_arg(0.70, 0.80, 1.00);
    cam.defocus_angle = 0.0;

    cam.render(&mut world);
    
}


pub fn earth() {
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg".to_string()));
    let earth_surface = Rc::new(RefCell::new(Lambertian::new_ptr(earth_texture)));
    let globe = Rc::new(RefCell::new(Sphere::new(Point3::new(), 2.0, earth_surface)));

    let mut cam = Camera::new();
    let world : Vec<Rc<RefCell<dyn Hittable>>> = vec![globe];

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 1200;
    cam.samples_per_pixel = 1000;
    cam.max_depth         = 50;

    cam.vfov     = 20.0;
    cam.lookfrom = Point3::new_arg(0.0,0.0,12.0);
    cam.lookat   = Point3::new();
    cam.vup      = Rvec3::new_arg(0.0,1.0,0.0);
    cam.background  = Color::new_arg(0.70, 0.80, 1.00);
    cam.defocus_angle = 0.0;

    cam.render(&mut HittableList::new_arg(world));
}

pub fn two_perlin_spheres() {
    let mut world = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new_arg(4.0));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg(0.0,-1000.0,0.0),1000.0, Rc::new(RefCell::new(Lambertian::new_ptr(pertext.clone())))))));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg(0.0,    2.0,0.0),   2.0, Rc::new(RefCell::new(Lambertian::new_ptr(pertext)))))));

    let mut cam = Camera::new();

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;

    cam.vfov     = 20.0;
    cam.lookfrom = Point3::new_arg(13.0,2.0,3.0);
    cam.lookat   = Point3::new();
    cam.vup      = Rvec3::new_arg(0.0,1.0,0.0);
    cam.background  = Color::new_arg(0.70, 0.80, 1.00);
    cam.defocus_angle = 0.0;

    cam.render(&mut world);
}

pub fn quads() {
    let mut world = HittableList::new();

    // Materials
    let left_red     = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(1.0, 0.2, 0.2))));
    let back_green   = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.2, 1.0, 0.2))));
    let right_blue   = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.2, 0.2, 1.0))));
    let upper_orange = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(1.0, 0.5, 0.0))));
    let lower_teal   = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.2, 0.8, 0.8))));

    // Quads
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(-3.0,-2.0, 5.0), Rvec3::new_arg(0.0, 0.0,-4.0), Rvec3::new_arg(0.0, 4.0, 0.0), left_red))));
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(-2.0,-2.0, 0.0), Rvec3::new_arg(4.0, 0.0, 0.0), Rvec3::new_arg(0.0, 4.0, 0.0), back_green))));
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg( 3.0,-2.0, 1.0), Rvec3::new_arg(0.0, 0.0, 4.0), Rvec3::new_arg(0.0, 4.0, 0.0), right_blue))));
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(-2.0, 3.0, 1.0), Rvec3::new_arg(4.0, 0.0, 0.0), Rvec3::new_arg(0.0, 0.0, 4.0), upper_orange))));    
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(-2.0,-3.0, 5.0), Rvec3::new_arg(4.0, 0.0, 0.0), Rvec3::new_arg(0.0, 0.0,-4.0), lower_teal))));


    let mut cam = Camera::new();

    cam.aspect_ratio      = 1.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;

    cam.vfov     = 80.0;
    cam.lookfrom = Point3::new_arg(0.0,0.0,9.0);
    cam.lookat   = Point3::new_arg(0.0,0.0,0.0);
    cam.vup      = Rvec3::new_arg(0.0,1.0,0.0);
    cam.background  = Color::new_arg(0.70, 0.80, 1.00);
    cam.defocus_angle = 0.0;

    cam.render(&mut world);
}

pub fn simple_light() {
    let mut world = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new_arg(4.0));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg(0.0,-1000.0, 0.0), 1000.0, Rc::new(RefCell::new(Lambertian::new_ptr(pertext.clone())))))));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg(0.0,    2.0, 0.0),    2.0, Rc::new(RefCell::new(Lambertian::new_ptr(pertext)))))));

    let difflight = Rc::new(RefCell::new(DiffuseLight::new_col(Color::new_arg(4.0, 4.0, 4.0))));
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(3.0, 1.0, -2.0), Rvec3::new_arg(2.0, 0.0, 0.0), Rvec3::new_arg(0.0, 2.0, 0.0), difflight.clone()))));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new_arg(0.0, 7.0, 0.0), 2.0, difflight))));

    let mut cam = Camera::new();

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;
    cam.background        = Color::new();

    cam.vfov     = 20.0;
    cam.lookfrom = Point3::new_arg(26.0,3.0,6.0);
    cam.lookat   = Point3::new_arg(0.0,2.0,0.0);
    cam.vup      = Rvec3::new_arg(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&mut world);
}

pub fn cornell_box() {
    let mut world = HittableList::new();

    let red   = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.65, 0.05, 0.05))));
    let white = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.73, 0.73, 0.73))));
    let green = Rc::new(RefCell::new(Lambertian::new(Color::new_arg(0.12, 0.45, 0.15))));
    let light = Rc::new(RefCell::new(DiffuseLight::new_col(Color::new_arg(15.0, 15.0, 15.0))));

    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(555.0,   0.0,   0.0), Rvec3::new_arg(   0.0, 555.0, 0.0), Rvec3::new_arg(0.0,  0.0, 555.0), green))));
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(  0.0,   0.0,   0.0), Rvec3::new_arg(   0.0, 555.0, 0.0), Rvec3::new_arg(0.0,  0.0, 555.0), red))));
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(343.0, 554.0, 332.0), Rvec3::new_arg(-130.0,   0.0, 0.0), Rvec3::new_arg(0.0,  0.0,-105.0), light))));
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(  0.0,   0.0,   0.0), Rvec3::new_arg( 555.0,   0.0, 0.0), Rvec3::new_arg(0.0,  0.0, 555.0), white.clone()))));
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(555.0, 555.0, 555.0), Rvec3::new_arg(-555.0,   0.0, 0.0), Rvec3::new_arg(0.0,  0.0,-555.0), white.clone()))));
    world.add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(  0.0,   0.0, 555.0), Rvec3::new_arg( 555.0,   0.0, 0.0), Rvec3::new_arg(0.0,555.0,   0.0), white.clone()))));

    //boxes
    let mut box1 : Rc<RefCell<dyn Hittable>> = HittableList::box_new(&mut Point3::new(), &mut Point3::new_arg(165.0, 330.0, 165.0), white.clone());
    box1 = Rc::new(RefCell::new(RotateY::new(box1,15.0)));
    box1 = Rc::new(RefCell::new(Translate::new(box1, Rvec3::new_arg(265.0, 0.0, 295.0))));
    world.add(box1);

    let mut box2 : Rc<RefCell<dyn Hittable>> = HittableList::box_new(&mut Point3::new(), &mut Point3::new_arg(165.0, 165.0, 165.0), white);
    box2 = Rc::new(RefCell::new(RotateY::new(box2,-18.0)));
    box2 = Rc::new(RefCell::new(Translate::new(box2, Rvec3::new_arg(130.0, 0.0, 65.0))));
    world.add(box2);

    let mut cam = Camera::new();

    cam.aspect_ratio      = 1.0;
    cam.image_width       = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth         = 50;
    cam.background        = Color::new();

    cam.vfov     = 40.0;
    cam.lookfrom = Point3::new_arg(278.0, 278.0, -800.0);
    cam.lookat   = Point3::new_arg(278.0, 278.0, 0.0);
    cam.vup      = Rvec3::new_arg(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&mut world);
}

pub fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    match 7 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => ()
    }
}
