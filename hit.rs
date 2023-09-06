use crate::rvec3::*;
use crate::Ray;


pub struct HitRecord{
    pub p : Point3,
    pub normal : Rvec3,
    pub t : f64,
    pub front_face : bool
}

impl HitRecord{
    pub fn set_face_normal(&mut self,r : &mut Ray, outward_normal : &mut Rvec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        
        self.front_face = Rvec3::dot(&r.direction(), &outward_normal) < 0.0;
        if self.front_face != false {
            self.normal = *outward_normal;
        }else {
            self.normal = -*outward_normal;
        }
    }
}


pub trait Hittable{
    fn hit(&self, ray: &mut Ray, ray_tmin : f64, ray_tmax : f64, rec: &mut HitRecord) -> bool;
}