use crate::hit::*;
use crate::hitlist::*;
use crate::aabb::AABB;

use crate::utility::*;
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;
use crate::interval::*;

pub struct BvhNode{
    bbox : AABB,    
    _left : Rc<RefCell<dyn Hittable>>,
    _right : Rc<RefCell<dyn Hittable>>
}


impl BvhNode {
    pub fn new(objects : &mut Vec<Rc<RefCell<dyn Hittable>>>, start : i32, end : i32) -> Self{
        //let mut objects = src_objects.clone(); // Create a modifiable array of the source scene objects

        let axis = random_int(0,2);


        type FunType = fn(&Rc<RefCell<(dyn Hittable + 'static)>>, &Rc<RefCell<(dyn Hittable + 'static)>>) -> Ordering;
        let mut comparator : FunType = BvhNode::box_x_compare;
        if axis == 1 { comparator = BvhNode::box_y_compare}
        else if axis == 2 { comparator = BvhNode::box_z_compare;}


        let lft : Rc<RefCell<dyn Hittable>>;
        let rght : Rc<RefCell<dyn Hittable>>;

        let object_span = end - start;
        if object_span == 1{
            lft = Rc::clone(&objects[start as usize]);
            rght = Rc::clone(&objects[start as usize]);
        }else if object_span == 2{
            if comparator(&Rc::clone(&objects[start as usize]), &Rc::clone(&objects[(start+1) as usize])) == Ordering::Less{
                lft = Rc::clone(&objects[start as usize]);
                rght = Rc::clone(&objects[(start+1) as usize]);
            }else{
                lft = Rc::clone(&objects[(start+1) as usize]);    
                rght = Rc::clone(&objects[start as usize]);
            }
        }else{
            objects.sort_by(comparator);
        
            let mid = start + object_span/2;
            lft = Rc::new(RefCell::new(BvhNode::new(objects,start,mid)));
            rght = Rc::new(RefCell::new(BvhNode::new(objects,mid+1,end))); 
        }

        let bbbox = AABB::new_boxes(lft.as_ref().borrow().bounding_box(), rght.as_ref().borrow().bounding_box());
          
        Self { bbox: bbbox, _left: lft, _right: rght }
    }

    pub fn new_list(mut list : HittableList) -> Self{
        let ln = list.objects.len() as i32;
        BvhNode::new(&mut list.objects, 0, ln)
    }

    pub fn box_compare(a : &Rc<RefCell<dyn Hittable>>, b : &Rc<RefCell<dyn Hittable>>, axis_index : i32) -> Ordering{
        if a.borrow_mut().bounding_box().axis(axis_index).min < b.borrow_mut().bounding_box().axis(axis_index).min{
            return Ordering::Less;
        }else if a.borrow_mut().bounding_box().axis(axis_index).min == b.borrow_mut().bounding_box().axis(axis_index).min{
            return Ordering::Equal;
        }
        Ordering::Greater
    }

    pub fn box_x_compare(a : &Rc<RefCell<dyn Hittable>>, b : &Rc<RefCell<dyn Hittable>>) -> Ordering{
        BvhNode::box_compare(a, b, 0)
    }


    pub fn box_y_compare(a : &Rc<RefCell<dyn Hittable>>, b  : &Rc<RefCell<dyn Hittable>>) -> Ordering{
        BvhNode::box_compare(a, b, 1)
    }


    pub fn box_z_compare(a : &Rc<RefCell<dyn Hittable>>, b : &Rc<RefCell<dyn Hittable>>) -> Ordering{
        BvhNode::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode{
    fn hit(&mut self, ray: &mut crate::ray::Ray, ray_t : &mut crate::interval::Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray,*ray_t) {
            return false;
        }
        
        let hit__left = self._left.borrow_mut().hit(ray,ray_t,rec);
        let mut mx = ray_t.max;
        if hit__left { mx = rec.t;}
        
        let hit__right = self._right.borrow_mut().hit(ray, &mut Interval::new_arg(ray_t.min, mx), rec);
        
        hit__left || hit__right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }    
}