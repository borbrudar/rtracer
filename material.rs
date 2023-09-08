use crate::hit::*;
use crate::ray::*;
use crate::color::*;

use crate::rvec3::*;

pub trait Material{
    fn scatter(&mut self,r_in : &mut Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool;
}


pub struct Lambertian{
    albedo : Color
}

impl Lambertian{
    pub fn new(a : Color) -> Self{
        Self{
            albedo : a
        }
    }
}

impl Material for Lambertian{
    fn scatter(&mut self,_r_in : &mut Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool{
        let mut scatter_direction = rec.normal + Rvec3::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new_arg(rec.p,scatter_direction);
        *attenuation = self.albedo;
        true
    }    
}


pub struct Metal{
    albedo : Color,
    fuzz : f64
}


impl Metal{
    pub fn new(a : Color, f : f64) -> Self{
        let mut ass = f;
        if ass > 1.0 { ass =1.0;}
        Self{
            albedo : a,
            fuzz : ass
        }
    }
}

impl Material for Metal{
  fn scatter(&mut self,r_in : &mut Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool{
        let reflected = Rvec3::reflect(Rvec3::unit_vector(&mut r_in.direction()),rec.normal);
        *scattered = Ray::new_arg(rec.p,reflected + self.fuzz * Rvec3::random_unit_vector());
        *attenuation = self.albedo;
        Rvec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}