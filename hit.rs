use rand::random;

use crate::rvec3::*;
use crate::Ray;
use crate::interval::*;
use crate::material::*;
use crate::texture::Texture;
use crate::utility::INFINITY;
use crate::utility::degrees_to_radians;
use crate::utility::random_double;
use std::borrow::BorrowMut;
use std::cell::Ref;
use std::convert::Infallible;
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

pub struct RotateY{
    object : Rc<RefCell<dyn Hittable>>,
    sin_theta : f64,
    cos_theta : f64,
    bbox : AABB,
}

impl RotateY{
    pub fn new(p : Rc<RefCell<dyn Hittable>>, angle : f64) -> Self{
        let radians = degrees_to_radians(angle);
        let _sin_theta = radians.sin();
        let _cos_theta = radians.cos();
        let mut _bbox = p.as_ref().borrow_mut().bounding_box();

        let mut min = Point3::new_arg( INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new_arg(-INFINITY,-INFINITY,-INFINITY);

        for i in 0..2{
            for j in 0..2{
                for k in 0..2{
                    let x = (i as f64)*_bbox.x.max + (1.0 - i as f64) * _bbox.x.min;
                    let y = (j as f64)*_bbox.y.max + (1.0 - j as f64) * _bbox.y.min;
                    let z = (k as f64)*_bbox.z.max + (1.0 - k as f64) * _bbox.z.min;
                    
                    let newx = _cos_theta * x + _sin_theta * z;
                    let newz = -_sin_theta * x + _cos_theta *z;

                    let tester = Rvec3::new_arg(newx, y, newz);

                    for c in 0..3{
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }


        _bbox = AABB::new_points(min, max);
        Self { object: p, sin_theta: _sin_theta, cos_theta: _cos_theta, bbox: _bbox}
    }
                                    
}

impl Hittable for RotateY{
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn hit(&mut self, r: &mut Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool {
        // Change the ray from world space to object space
        let mut origin = r.origin().clone();
        let mut direction = r.direction().clone();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let mut rotated_r = Ray::new_time(origin,direction,r.time());

        // Determine where (if any) an intersection occurs in object space
        if !self.object.as_ref().borrow_mut().hit(&mut rotated_r, ray_t,rec){
            return false;
        } 
        
        // Change the intersection point from object space to world space
        let mut p = rec.p.clone();
        p[0] =  self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];


        // Change the normal from object space to world space
        let mut normal = rec.normal.clone();
        normal[0] =  self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];


        rec.p = p;
        rec.normal = normal;

        true
    }
}


pub struct ConstantMedium{
    boundary : Rc<RefCell<dyn Hittable>>,
    neg_inv_density : f64,
    phase_function : Rc<RefCell<dyn Material>>,
}

impl ConstantMedium{
    pub fn new_tex(b : Rc<RefCell<dyn Hittable>>, d : f64, a : Rc<dyn Texture>) -> Self{
        Self { boundary: b, neg_inv_density: -1.0/d, phase_function: Rc::new(RefCell::new(Isotropic::new_tex(a))) }
    }
    pub fn new_col(b : Rc<RefCell<dyn Hittable>>, d : f64, c : Color) -> Self{
        Self { boundary: b, neg_inv_density: -1.0/d, phase_function: Rc::new(RefCell::new(Isotropic::new(c))) }
    }
}

impl Hittable for ConstantMedium{
    fn bounding_box(&self) -> AABB {
        self.boundary.borrow().bounding_box()        
    }

    fn hit(&mut self, r: &mut Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        let enableDebug = false;
        let debugging = enableDebug && random_double() < 0.00001;

        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.as_ref().borrow_mut().hit(r, &mut Interval::UNIVERSE, &mut rec1) { return false;} 
        if !self.boundary.as_ref().borrow_mut().hit(r, &mut Interval::new_arg(rec1.t + 0.0001, INFINITY), &mut rec2) {return false;}

        if debugging {
            eprintln!("ray_tmin={}, ray_tmax={}", &rec1.t,&rec2.t);
        }

        if rec1.t < ray_t.min {rec1.t = ray_t.min;}
        if rec2.t > ray_t.max {rec2.t = ray_t.max;}

        if rec1.t >= rec2.t {return false;}
        if rec1.t < 0.0 { rec1.t = 0.0;}

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().log10();

        if hit_distance > distance_inside_boundary {return false;}

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!("hit_distance= {} \nrec.t = {} \n rec.p = {} ", &hit_distance, &rec.t, &rec.p);
        }

        rec.normal = Rvec3::new_arg(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; //also arbitrary
        rec.mat = Rc::clone(&self.phase_function);

        true
    }
}