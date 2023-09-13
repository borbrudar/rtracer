use crate::material::*;
use crate::aabb::*;
use std::rc::Rc;
use crate::rvec3::*;
use crate::hit::*;

pub struct Quad{
    Q : Point3,
    u : Rvec3,
    v : Rvec3,
    mat : Rc<dyn Material>,
    bbox : AABB,
}

impl Quad{
    pub fn new(_Q : Point3, _u : Rvec3, _v : Rvec3, m : Rc<dyn Material>) -> Self{
        Self{
            Q : _Q,
            u : _u,
            v : _v,
            mat : m,
            bbox : AABB::new_points(_Q,_Q+_u+_v).pad() // set_bounding_box()
        }
    }
}

impl Hittable for Quad{
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn hit(&self, ray: &mut crate::ray::Ray, ray_t : &mut crate::interval::Interval, rec: &mut HitRecord) -> bool {
        false 
    }      
}