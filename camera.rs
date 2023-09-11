use crate::utility::*;
use crate::color::*;
use crate::interval::*;
use crate::rvec3::*;
use crate::hit::*;
use crate::hitlist::*;
use crate::ray::*;

use crate::material::*;
use std::time::{Duration,Instant};

pub struct Camera{
    pub aspect_ratio : f64,  // Ratio of image width over height
    pub image_width : i32,  // Rendered image width in pixel count
    pub samples_per_pixel : i32, // anti-aliasing
    pub max_depth : i32, // max depth for recursion
    
    pub vfov : f64, //vertifcal field of view
    pub lookfrom : Point3, // Point camera is looking from
    pub lookat : Point3, // Point camera is looking at
    pub vup : Rvec3,   // Camera-relative "up" direction

    pub defocus_angle : f64, // Variation angle of rays through each pixel
    pub focus_dist : f64, // Distance from camera lookfrom point to plane of perfect focus

    image_height : i32,   // Rendered image height
    center : Point3,         // Camera center
    pixel00_loc : Point3,    // Location of pixel 0, 0
    pixel_delta_u : Rvec3,  // Offset to pixel to the right
    pixel_delta_v : Rvec3,  // Offset to pixel below

    w : Rvec3, //Basis vectors
    u : Rvec3, 
    v : Rvec3,

    defocus_disk_u : Rvec3, // Defocus disk horizontal radius
    defocus_disk_v : Rvec3, // Defocus disk vertical radius
}

impl Default for Camera{
    fn default() -> Self{
        Self::new()
    }
}

impl Camera{
    pub fn new() -> Camera{
        Self{
            aspect_ratio : 1.0,
            image_width : 100,
            samples_per_pixel : 10,
            max_depth : 10,
            vfov : 90.0, 
            lookfrom : Point3::new_arg(0.0,0.0,-1.0),
            lookat : Point3::new_arg(0.0,0.0,0.0),
            vup : Rvec3::new_arg(0.0,1.0,0.0),

            defocus_angle : 0.0,
            focus_dist : 10.0,
            // made up
            image_height : 0,
            center : Point3::new(),
            pixel00_loc : Point3::new(),
            pixel_delta_u : Point3::new(),
            pixel_delta_v : Point3::new(),

            w : Rvec3::new(),
            u : Rvec3::new(),
            v : Rvec3::new(),

            defocus_disk_u : Rvec3::new(),
            defocus_disk_v : Rvec3::new(),
        }
    }

    pub fn render(&mut self, world : &mut HittableList) {
        self.initialize();
    
        // Render                         
        println!("P3\n{} {}\n255", &self.image_width,&self.image_height);
    
        //timing
        let start = Instant::now();
        
        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {}", self.image_height-j); 
            for i in 0..self.image_width{
                let mut pixel_color = Color::new_arg(0.0,0.0,0.0);
                for _sample in 0..self.samples_per_pixel {
                    let mut r = self.get_ray(i,j); 
                    pixel_color += self.ray_color(&mut r, self.max_depth, world);
                }
                
                write_color(&mut pixel_color, self.samples_per_pixel);
            }
        }
        
        eprintln!("\rDone");
        let duration = start.elapsed();
        eprintln!("Time elapsed in expensive_function() is: {:?}", duration);
    }

    fn get_ray(&mut self, i : i32, j : i32) -> Ray{
        // Get a randomly sampled camera ray for the pixel at location i,j.
        let pixel_center = self.pixel00_loc + ( (i as f64)* self.pixel_delta_u) + ((j as f64) * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;

        // turn on defocus blur (ray_origin must be mut)
        /*
        if self.defocus_angle > 0.0{
            ray_origin = self.defocus_disk_sample();
        }
        */
        
        let ray_direction = Rvec3::unit_vector(&mut (pixel_sample - ray_origin));
        let ray_time = random_double();

        Ray::new_time(ray_origin,ray_direction,ray_time)
    }

    fn pixel_sample_square(&mut self) -> Rvec3{
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn initialize(&mut self){
        // Calculate the image height, and ensure that it's at least 1.
        self.image_height =  ((self.image_width as f64)/self.aspect_ratio) as i32;
        if self.image_height < 1 {self.image_height = 1;}
        
        self.center = self.lookfrom;

        // Determine viewport dimensions.
        //let focal_length = (self.lookfrom - self.lookat).length();
        let theta = self.vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h  * self.focus_dist;
        let viewport_width = viewport_height * ((self.image_width as f64)/(self.image_height as f64));
        

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = Rvec3::unit_vector(&mut (self.lookfrom - self.lookat));
        self.u = Rvec3::unit_vector(&mut Rvec3::cross(&self.vup, &self.w));
        self.v = Rvec3::cross(&self.w, &self.u);

        
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;    // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v;  // Vector down viewport vertical edge
        
        
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);
        
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - (self.focus_dist * self.w) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&mut self,r : &mut Ray, depth : i32, world : &mut HittableList) -> Color {
        let mut rec : HitRecord = HitRecord::new(); 

        if depth <= 0 {
            return Color::new();
        }

        if world.hit(r, &mut Interval{min : 0.001, max : INFINITY} , &mut rec){
            //let direction = Rvec3::random_on_hemisphere(&rec.normal); // uniform random distribution
            //let direction = rec.normal + Rvec3::random_unit_vector(); // lambertian distribution

            let mut scattered = Ray::new();
            let mut attenuation = Color::new();
            if rec.mat.borrow_mut().scatter(r, &rec, &mut attenuation,&mut scattered) {
                return attenuation * self.ray_color(&mut scattered,depth-1,world);
            }   
            return Color::new_arg(0.0,0.0,0.0);
        }
        
        let mut unit_direction = Rvec3::unit_vector(&mut r.direction());
        let a : f64 = 0.5*(unit_direction.y() + 1.0);
        (1.0-a)*Color::new_arg(1.0,1.0,1.0) + a*Color::new_arg(0.5,0.7,1.0)
    }

    pub fn defocus_disk_sample(&self)  -> Point3{
        // Returns a random point in the camera defocus disk.
        let p = Rvec3::random_in_unit_disk();
        self.center + (p.e[0] * self.defocus_disk_u) + (p.e[1] * self.defocus_disk_v)
    }

}