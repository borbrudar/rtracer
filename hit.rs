use crate::rvec3::*;
use crate::Ray;
use crate::interval::*;
use crate::material::*;
use std::borrow::BorrowMut;
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
    fn hit(&mut self, ray: &mut Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}

pub struct Translate{
    object : Rc<RefCell<dyn Hittable>>,
    offset : Rvec3,
    bbox : AABB,    
}


impl Translate{
    pub fn new(p : Rc<RefCell<dyn Hittable>>, displacement : Rvec3) -> Self{
        Self { 
            object : p.clone(),
            offset : displacement,
            bbox : p.borrow().bounding_box().clone() + displacement
        }
    }
}

impl Hittable for Translate{
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn hit(&mut self, r: &mut Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let mut offset_r = Ray::new_time(r.origin() - self.offset, r.direction(), r.time());
        
        // Determine where (if any) an intersection occurs along the offset ray
        if !self.object.as_ref().borrow_mut().hit(&mut offset_r, ray_t, rec){
            return false;
        }


        // Move the intersection point forwards by the offset
        rec.p += self.offset;

        true        
    }   
}