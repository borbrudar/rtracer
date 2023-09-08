use crate::color::*;
use crate::hit::*;
use crate::ray::*;
use crate::ray::*;
use crate::rvec3::*;

pub trait Material {
    fn scatter(
        &mut self,
        r_in: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &mut self,
        r_in: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        //eprintln!("lol");
        let mut scatter_direction = rec.normal + Rvec3::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new_arg(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Material for Metal {
    fn scatter(
        &mut self,
        r_in: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        eprintln!("lol");
        let reflected = Rvec3::reflect(Rvec3::unit_vector(&mut r_in.direction()), rec.normal);
        *scattered = Ray::new_arg(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}
