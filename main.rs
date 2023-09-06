pub mod rvec3;
pub mod color;
pub mod ray;
pub mod hit;
pub mod sphere;

use rvec3::*;
use color::*;
use ray::*;

pub fn hit_sphere(center : Point3, radius : f64, r : &mut Ray) -> f64 {
    let mut oc : Rvec3 = r.origin() - center;

    let a = r.direction().length_squared();
    let half_b = Rvec3::dot(&oc, &r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant : f64 = half_b*half_b - a*c;
    if discriminant < 0.0{
        -1.0
    }else{
        (-half_b - discriminant.sqrt())/ a
    }
}

pub fn ray_color(r : &mut Ray) -> Color {
    let t =  hit_sphere(Point3::new_arg(0.0,0.0,-1.0),0.5,r);
        
    if t > 0.0{
        let mut n = Rvec3::unit_vector(&mut (r.at(t) - Rvec3::new_arg(0.0,0.0,-1.0)));
        return 0.5 * Color::new_arg(n.x()+1.0, n.y()+1.0, n.z() + 1.0);
    }

    let mut unit_direction = Rvec3::unit_vector(&mut r.direction());
    let a : f64 = 0.5*(unit_direction.y() + 1.0);
    (1.0-a)*Color::new_arg(1.0,1.0,1.0) + a*Color::new_arg(0.5,0.7,1.0)
}

pub fn main(){
    //image
    let aspect_ratio : f64 = 16.0 / 9.0;
    let image_width : i32= 400;
    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height : i32=  ((image_width as f64)/aspect_ratio) as i32;
    if image_height < 1 {image_height = 1;}
    
    
    // Camera
    let focal_length : f64 = 1.0;
    let viewport_height : f64 = 2.0;
    let viewport_width = viewport_height * ((image_width as f64)/(image_height as f64));
    let camera_center = Point3::new();
    

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Rvec3::new_arg(viewport_width, 0.0, 0.0);
    let viewport_v = Rvec3::new_arg(0.0, -viewport_height, 0.0);
    
    
    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);
    
    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center- Rvec3::new_arg(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0; 
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    

 // Render
                             
    println!("P3\n{} {}\n255", &image_width,&image_height);
 

    // Render
    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height-j); 
        for i in 0..image_width{
            let pixel_center : Point3 = pixel00_loc + ((i as f64) * pixel_delta_u) + ((j as f64)*pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let mut r : Ray = Ray::new_arg(camera_center,ray_direction);

            let mut pixel_color = ray_color(&mut r);
            write_color(&mut pixel_color);
        }
    }

    eprintln!("\rDone");
}
