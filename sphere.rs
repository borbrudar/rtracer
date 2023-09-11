use crate::hit::*;
use crate::rvec3::*;
use crate::ray::*;
use crate::interval::*;
use crate::material::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::aabb::AABB;

pub struct Sphere {
    center : Point3,
    radius : f64,
    mat : Rc<RefCell<dyn Material>>,

    is_moving : bool,
    center_vec : Rvec3,
    bbox : AABB,
}


impl Sphere{
    pub fn new(cnt : Point3, rad : f64, mt : Rc<RefCell<dyn Material>>) -> Self{
        let rvec = Rvec3::new_arg(rad, rad, rad);
        Self{
            center : cnt,
            radius : rad,
            mat : mt,
            is_moving : false,
            center_vec : Rvec3::new(),
            bbox : AABB::new_points(cnt - rvec, cnt + rvec)
        }
    }
    pub fn new_movable(cnt : Point3, cnt2 : Point3, rad : f64, mt : Rc<RefCell<dyn Material>>) -> Self{
        let rvec = Rvec3::new_arg(rad, rad, rad);
        let box1 = AABB::new_points(cnt - rvec, cnt + rvec);
        let box2 = AABB::new_points(cnt2 - rvec, cnt2 + rvec);
        Self { 
            center:cnt, 
            radius: rad, 
            mat: mt, 
            is_moving: true, 
            center_vec: cnt2-cnt,
            bbox : AABB::new_boxes(box1,box2),
        }
    }

    pub fn sphere_center(&self, time : f64) -> Point3{
        // Linearly interpolate from center1 to center2 according to time, where t=0 yields
        // center1, and t=1 yields center2.
        self.center + time*self.center_vec
    }

    pub fn get_sphere_uv(mut p : Point3, u : &mut f64, v : &mut f64){
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;

        *u = phi / (2.0*std::f64::consts::PI);
        *v = theta / phi;
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
        Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}