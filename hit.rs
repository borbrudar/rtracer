use crate::rvec3::*;
use crate::Ray;


pub struct HitRecord{
    pub p : Point3,
    pub normal : Rvec3,
    pub t : f64
}


pub trait Hittable{
    fn hit(&self, ray: &mut Ray, ray_tmin : f64, ray_tmax : f64, rec: &mut HitRecord) -> bool;
}