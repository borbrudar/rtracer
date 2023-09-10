use crate::rvec3::*;
use crate::Ray;
use crate::interval::*;
use crate::material::*;
use std::rc::Rc;
use crate::color::*;
use std::cell::RefCell;
use crate::aabb::AABB;

pub struct HitRecord{
    pub p : Point3,
    pub normal : Rvec3,
    pub t : f64,
    pub front_face : bool,
    pub mat : Rc<RefCell<dyn Material>>,
    pub v : f64, // coord mappings
    pub u : f64,
}

impl HitRecord{
    pub fn new() -> Self{
        Self{
            p : Point3::new(),
            normal : Rvec3::new(),
            t : 0.0,
            front_face : false,
            mat : Rc::new(RefCell::new(Lambertian::new(Color::new_arg(251.0,0.0,120.0)))),
            v : 0.0,
            u : 0.0,
        }
    }

    pub fn set_face_normal(&mut self,r : &mut Ray, outward_normal : &mut Rvec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        
        self.front_face = Rvec3::dot(&r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        }else {
            self.normal = -*outward_normal;
        }
    }
}

impl Default for HitRecord{
    fn default() -> Self{
        todo!()
    }       
}

pub trait Hittable{
    fn hit(&self, ray: &mut Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}