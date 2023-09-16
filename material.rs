use crate::hit::*;
use crate::ray::*;
use crate::color::*;
use crate::utility::random_double;
use crate::rvec3::*;

use std::rc::Rc;
use crate::texture::*;
use crate::perlin::*;

pub trait Material{
    fn scatter(&mut self,r_in : &mut Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool;
    fn emitted(&mut self, _u : f64, _v : f64, _p : &Point3) -> Color {
        Color::new()
    }
}


pub struct Lambertian{
    albedo : Rc<dyn Texture>
}

impl Lambertian{
    pub fn new(a : Color) -> Self{
        Self{
            albedo : Rc::new(SolidColor::new(a))
        }
    }
    pub fn new_ptr(a : Rc<dyn Texture>) -> Self {
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

        *scattered = Ray::new_time(rec.p,scatter_direction,_r_in.time());
        *attenuation = self.albedo.value(rec.u,rec.v,rec.p);
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
        *scattered = Ray::new_time(rec.p,reflected + self.fuzz * Rvec3::random_unit_vector(),r_in.time());
        *attenuation = self.albedo;
        Rvec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}


pub struct Dielectric{
    ir : f64, // index of refraction
}

impl Dielectric{
    pub fn new(id : f64) -> Self{
        Self{
            ir : id
        }
    }

    pub fn reflectance(cosine : f64, ref_idx : f64) -> f64{
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0-ref_idx)/(1.0+ref_idx);
        r0 *= r0;
        r0 + (1.0-r0)*(1.0-cosine).powf(5.0)       
    }
}

impl Material for Dielectric{
    fn scatter(&mut self,r_in : &mut Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool{
        *attenuation = Color::new_arg(1.0,1.0,1.0);
        let mut refraction_ratio = self.ir;
        if rec.front_face { refraction_ratio = 1.0/self.ir;}

        let unit_direction = Rvec3::unit_vector(&mut r_in.direction());
        let cos_theta = Rvec3::dot(&-unit_direction,&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
 
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction : Rvec3 = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double() {
            Rvec3::reflect(unit_direction,rec.normal)
        }else{ 
            Rvec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray::new_time(rec.p, direction,r_in.time());
        true
    }
}

pub struct NoiseTexture{
    noise : Perlin,
    scale : f64,
}

impl Default for NoiseTexture{
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseTexture{
    pub fn new() -> Self{
        Self { noise: Perlin::new()  , scale : 1.0 }
    }
    pub fn new_arg(sc : f64) -> Self{
        Self { noise:Perlin::new(), scale: sc }
    }
}

impl Texture for NoiseTexture{
    fn value(&self, _u : f64, _v : f64, p : Point3) -> Color {
        let mut s = self.scale * p;
        Color::new_arg(1.0, 1.0, 1.0) * 0.5  * (1.0 + s.z().sin() + 10.0 * self.noise.turb(&s,7))
    }
}


pub struct DiffuseLight {
    emit : Rc<dyn Texture>,
}

impl DiffuseLight{
    pub fn new(a : Rc<dyn Texture>) -> Self{
        Self {
            emit : a,
        }
    }

    pub fn new_col(c : Color) -> Self{
        Self {
            emit : Rc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight{
    fn scatter(&mut self,_r_in : &mut Ray, _rec : &HitRecord, _attenuation : &mut Color, _scattered : &mut Ray) -> bool {
        false
    }
    fn emitted(&mut self, u : f64, v : f64, p : &Point3) -> Color {
        self.emit.value(u,v,*p)
    }
}


pub struct Isotropic{
    albedo : Rc<dyn Texture>,
}

impl Isotropic{
    pub fn new(c : Color) -> Self{
        Self { albedo: Rc::new(SolidColor::new(c)) }
    }
    pub fn new_tex(a : Rc<dyn Texture>) -> Self{
        Self { albedo:  a }
    }
}

impl Material for Isotropic{
    fn scatter(&mut self,r_in : &mut Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool {
        *scattered = Ray::new_time(rec.p, Rvec3::random_unit_vector(), r_in.time());
        *attenuation = self.albedo.value(rec.u,rec.v, rec.p); 
        true   
    }
}