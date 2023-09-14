use crate::hit::*;
use crate::material::Material;
use crate::ray::*;
use crate::interval::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::rvec3::*;
use crate::quad::*;
use crate::aabb::*;

pub struct HittableList {
    pub objects : Vec<Rc<RefCell<dyn Hittable>>> 
}

impl HittableList{
    pub fn new() -> Self{
        Self{
            objects : Vec::new()
        }
    }
    pub fn new_arg(obj : Vec<Rc<RefCell<dyn Hittable>>> ) -> Self{
        Self { 
            objects: obj 
        }
    }

    pub fn clear(&mut self){
        self.objects.clear();
    }

    pub fn add(&mut self, object : Rc<RefCell<dyn Hittable>>){
        self.objects.push(object);
    }

    pub fn box_new(a : &mut Point3, b : &mut Point3, mat : Rc<RefCell<dyn Material>>) -> Rc<RefCell<HittableList>> {
        // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
        let sides = Rc::new(RefCell::new(HittableList::new()));

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let mut min = Point3::new_arg(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let mut max = Point3::new_arg(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Rvec3::new_arg(max.x() - min.x(), 0.0, 0.0);
        let dy = Rvec3::new_arg(0.0, max.y() - min.y(), 0.0);
        let dz = Rvec3::new_arg(0.0, 0.0, max.z() - min.z());

        sides.borrow_mut().add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(min.x(), min.y(), max.z()),  dx,  dy, mat.clone())))); // front    
        sides.borrow_mut().add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(max.x(), min.y(), max.z()), -dz,  dy, mat.clone())))); // right
        sides.borrow_mut().add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(max.x(), min.y(), min.z()), -dx,  dy, mat.clone())))); // back    
        sides.borrow_mut().add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(min.x(), min.y(), min.z()),  dz,  dy, mat.clone())))); // left    
        sides.borrow_mut().add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(min.x(), max.y(), max.z()),  dx, -dz, mat.clone())))); // top    
        sides.borrow_mut().add(Rc::new(RefCell::new(Quad::new(Point3::new_arg(min.x(), min.y(), min.z()),  dx,  dz, mat.clone())))); // bottom    

        sides
    }
}

impl Default for HittableList {
    fn default() -> Self {
            Self::new()
    }
}

impl Hittable for HittableList{
    fn hit(&mut self, ray: &mut Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything  = false;
        let mut closest_so_far = ray_t.max;
    
        for object in self.objects.iter(){
            let mut temp_rec : HitRecord = HitRecord::new();
            if object.borrow_mut().hit(ray, &mut Interval{min : ray_t.min, max : closest_so_far} ,&mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
    fn bounding_box(&self) -> crate::aabb::AABB {
        AABB::new()       
    }
}