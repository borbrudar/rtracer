use crate::hit::*;
use crate::hitlist::*;
use crate::aabb::AABB;
use crate::interval::*;
use crate::utility::*;
use std::rc::Rc;

pub struct BvhNode{
    bbox : AABB,    
    left : Rc<dyn Hittable>,
    right : Rc<dyn Hittable>
}


impl BvhNode {
    pub fn new(src_objects : &Vec<Rc<dyn Hittable>>, start : i32, end : i32) -> Self{
        let objects = src_objects; // Create a modifiable array of the source scene objects

        let axis = random_int(0,2);


        type FunType = fn(Rc<(dyn Hittable + 'static)>, Rc<(dyn Hittable + 'static)>) -> bool;
        let mut comparator : FunType = BvhNode::box_x_compare;
        if axis == 1 { comparator = BvhNode::box_y_compare}
        else if axis == 2 { comparator = BvhNode::box_z_compare;}


        let lft : Rc<dyn Hittable>;
        let rght : Rc<dyn Hittable>;

        let object_span = end - start;
        if object_span == 1{
            lft = Rc::clone(&objects[start as usize]);
            rght = Rc::clone(&objects[start as usize]);
        }else if object_span == 2{
            if comparator(Rc::clone(&objects[start as usize]), Rc::clone(&objects[(start+1) as usize])){
                lft = Rc::clone(&objects[start as usize]);
                rght = Rc::clone(&objects[(start+1) as usize]);
            }else{
                lft = Rc::clone(&objects[(start+1) as usize]);    
                rght = Rc::clone(&objects[start as usize]);
            }
        }else{
            //objects.sort_by(comparator);
        
            let mid = start + object_span/2;
            lft = Rc::new(BvhNode::new(objects,start,mid));
            rght = Rc::new(BvhNode::new(objects,mid+1,end)); 
        }

        let bbbox = AABB::new_boxes(lft.bounding_box(), rght.bounding_box());
          
        Self { bbox: bbbox, left: lft, right: rght }
    }

    pub fn new_list(list : HittableList) -> Self{
        let ln = list.objects.len() as i32;
        BvhNode::new(&list.objects, 0, ln)
    }

    pub fn box_compare(a : Rc<dyn Hittable>, b : Rc<dyn Hittable>, axis_index : i32) -> bool{
        a.bounding_box().axis(axis_index).min < b.bounding_box().axis(axis_index).min
    }

    pub fn box_x_compare(a : Rc<dyn Hittable>, b : Rc<dyn Hittable>) -> bool{
        BvhNode::box_compare(a, b, 0)
    }


    pub fn box_y_compare(a : Rc<dyn Hittable>, b : Rc<dyn Hittable>) -> bool{
        BvhNode::box_compare(a, b, 1)
    }


    pub fn box_z_compare(a : Rc<dyn Hittable>, b : Rc<dyn Hittable>) -> bool{
        BvhNode::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode{
    fn hit(&self, ray: &mut crate::ray::Ray, ray_t : &mut crate::interval::Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray,*ray_t) {
            return false;
        }

        let hit_left = self.left.hit(ray,ray_t,rec);
        let mut mx = ray_t.max;
        if hit_left { mx = rec.t;}

        let hit_right = self.right.hit(ray, &mut Interval::new_arg(ray_t.min, mx), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }    
}