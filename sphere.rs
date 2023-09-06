use crate::hit::*;
use crate::rvec3::*;
use crate::ray::*;

pub struct Sphere {
    center : Point3,
    radius : f64
}


impl Sphere{
    pub fn new(cnt : Point3, rad : f64) -> Self{
        Self{
            center : cnt,
            radius : rad
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: &mut Ray, ray_tmin : f64, ray_tmax : f64, rec: &mut HitRecord) -> bool{
        let mut oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Rvec3::dot(&oc,&r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant : f64 = half_b*half_b - a*c;
        if discriminant < 0.0 {return false;}
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd)/a;
        if root <= ray_tmin || ray_tmax <= root{
            root = (-half_b + sqrtd)/a;
            if root <= ray_tmin || ray_tmax <= root { return false;} 
        }


        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}