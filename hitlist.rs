use crate::hit::*;
use crate::ray::*;
use crate::interval::*;


pub struct HittableList {
    objects : Vec<Box<dyn Hittable>> 
}

impl HittableList{
    pub fn new() -> Self{
        Self{
            objects : Vec::new()
        }
    }
    pub fn clear(&mut self){
        self.objects.clear();
    }

    pub fn add(&mut self, object : Box<dyn Hittable>){
        self.objects.push(object);
    }

    pub fn hit(&mut self, r : &mut Ray, ray_t : &mut Interval, rec : &mut HitRecord) -> bool{
        let mut hit_anything  = false;
        let mut closest_so_far = ray_t.max;
        
        for object in self.objects.iter(){
            let mut temp_rec : HitRecord = HitRecord::new();
            if object.hit(r, &mut Interval{min : ray_t.min, max : closest_so_far} ,&mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}

impl Default for HittableList {
    fn default() -> Self {
            Self::new()
    }
}