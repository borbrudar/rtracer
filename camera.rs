use crate::utility::*;
use crate::color::*;
use crate::interval::*;
use crate::rvec3::*;
use crate::hit::*;
use crate::hitlist::*;
use crate::ray::*;

use crate::material::*;

pub struct Camera{
    pub aspect_ratio : f64,  // Ratio of image width over height
    pub image_width : i32,  // Rendered image width in pixel count
    pub samples_per_pixel : i32, // anti-aliasing
    pub max_depth : i32, // max depth for recursion

    image_height : i32,   // Rendered image height
    camera_center : Point3,         // Camera center
    pixel00_loc : Point3,    // Location of pixel 0, 0
    pixel_delta_u : Rvec3,  // Offset to pixel to the right
    pixel_delta_v : Rvec3,  // Offset to pixel below
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
            // made up
            image_height : 0,
            camera_center : Point3::new(),
            pixel00_loc : Point3::new(),
            pixel_delta_u : Point3::new(),
            pixel_delta_v : Point3::new()
        }
    }

    pub fn render(&mut self, world : &mut HittableList) {
        self.initialize();
    
        // Render                         
        println!("P3\n{} {}\n255", &self.image_width,&self.image_height);
    
        // Render
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
    }

    fn get_ray(&mut self, i : i32, j : i32) -> Ray{
        // Get a randomly sampled camera ray for the pixel at location i,j.
        let pixel_center = self.pixel00_loc + ( (i as f64)* self.pixel_delta_u) + ((j as f64) * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new_arg(ray_origin,ray_direction)
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
        
        
        // Camera
        let focal_length : f64 = 1.0;
        let viewport_height : f64 = 2.0;
        let viewport_width = viewport_height * ((self.image_width as f64)/(self.image_height as f64));
        self.camera_center = Point3::new();
        

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Rvec3::new_arg(viewport_width, 0.0, 0.0);
        let viewport_v = Rvec3::new_arg(0.0, -viewport_height, 0.0);
        
        
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);
        
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.camera_center- Rvec3::new_arg(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0; 
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
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
}