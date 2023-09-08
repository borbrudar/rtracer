use crate::hit::*;
use crate::rvec3::*;
use crate::ray::*;
use crate::interval::*;
use crate::material::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Sphere {
    center : Point3,
    radius : f64,
    mat : Rc<RefCell<dyn Material>>
}


impl Sphere{
    pub fn new(cnt : Point3, rad : f64, mt : Rc<RefCell<dyn Material>>) -> Self{
        Self{
            center : cnt,
            radius : rad,
            mat : mt
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: &mut Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool{
        let mut oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Rvec3::dot(&oc,&r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant : f64 = half_b*half_b - a*c;
        if discriminant < 0.0 {return false;}
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd)/a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd)/a;
            if !ray_t.surrounds(root) { return false;} 
        }


        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        rec.mat = Rc::clone(&self.mat);

        let mut outward_normal = rec.normal;
        rec.set_face_normal(r, &mut outward_normal);

        true
    }
}