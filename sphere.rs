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
    mat : Rc<RefCell<dyn Material>>,

    is_moving : bool,
    center_vec : Rvec3,
}


impl Sphere{
    pub fn new(cnt : Point3, rad : f64, mt : Rc<RefCell<dyn Material>>) -> Self{
        Self{
            center : cnt,
            radius : rad,
            mat : mt,
            is_moving : false,
            center_vec : Rvec3::new()
        }
    }
    pub fn new_movable(cnt : Point3, cnt2 : Point3, rad : f64, mt : Rc<RefCell<dyn Material>>) -> Self{
        Self { 
            center:cnt, 
            radius: rad, 
            mat: mt, 
            is_moving: true, 
            center_vec: cnt2-cnt
        }
    }

    pub fn sphere_center(&self, time : f64) -> Point3{
        // Linearly interpolate from center1 to center2 according to time, where t=0 yields
        // center1, and t=1 yields center2.
        self.center + time*self.center_vec
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: &mut Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool{
        let mut calc_center = self.center;
        if self.is_moving {
            calc_center = self.sphere_center(r.time());
        }
        let mut oc = r.origin() - calc_center;
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
        let mut outward_normal = (rec.p - calc_center) / self.radius;
        rec.set_face_normal(r, &mut outward_normal);
        
        rec.mat = Rc::clone(&self.mat);
        
        true
    }
}