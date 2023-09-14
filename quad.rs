use crate::material::*;
use crate::aabb::*;
use std::rc::Rc;
use crate::rvec3::*;
use crate::hit::*;
use std::cell::RefCell;

pub struct Quad{
    q : Point3,
    u : Rvec3,
    v : Rvec3,
    mat : Rc<RefCell<dyn Material>>,
    bbox : AABB,
    normal : Rvec3,
    d : f64,
    w : Rvec3,
}

impl Quad{
    pub fn new(_q : Point3, _u : Rvec3, _v : Rvec3, m : Rc<RefCell<dyn Material>>) -> Self{
        let mut n = Rvec3::cross(&_u,&_v);
        let norm = Rvec3::unit_vector(&mut n);
        let _d = Rvec3::dot(&norm, &_q);
        let _w = n / Rvec3::dot(&n,&n);
        Self{
            q : _q,
            u : _u,
            v : _v,
            mat : m,
            bbox : AABB::new_points(_q,_q+_u+_v).pad(), // set_bounding_box()
            normal : norm,
            d : _d,
            w : _w,
        }
    }

    pub fn is_interior(a : f64, b : f64, rec : &mut HitRecord) -> bool{
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.
        if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b){
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad{
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn hit(&mut self, ray: &mut crate::ray::Ray, ray_t : &mut crate::interval::Interval, rec: &mut HitRecord) -> bool {
        let denom = Rvec3::dot(&self.normal, &ray.direction());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 0.00000001 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - Rvec3::dot(&self.normal, &ray.origin())) / denom;
        if !ray_t.contains(t) {    
            return false;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = Rvec3::dot(&self.w, &Rvec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Rvec3::dot(&self.w, &Rvec3::cross(&self.u, &planar_hitpt_vector));

        if !Quad::is_interior(alpha, beta, rec){
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        rec.t = t;
        rec.p = intersection;
        rec.mat = Rc::clone(&self.mat);
        rec.set_face_normal(ray, &mut self.normal);

        true
    }      
}