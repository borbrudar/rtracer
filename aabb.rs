use crate::interval::*;
use crate::rvec3::*;
use crate::ray::*;

#[derive(Clone, Copy)]
pub struct AABB{
    pub x : Interval,
    pub y : Interval,
    pub z : Interval,
}

impl AABB{
    //empty bounding box
    pub fn new() -> Self{
        Self{
            x : Interval::new(),
            y : Interval::new(),
            z : Interval::new(),
        }
    }

    pub fn new_arg(ix : Interval, iy : Interval, iz : Interval) -> Self{
        Self{
            x : ix,
            y : iy,
            z : iz,
        }
    }

    pub fn new_boxes(box0 : AABB, box1 : AABB) -> Self{
        Self { 
            x : Interval::new_intervals(box0.x, box1.x),
            y : Interval::new_intervals(box0.y,box1.y),
            z : Interval::new_intervals(box0.z,box1.z),                       
        }
    }

    pub fn new_points(a : Point3, b : Point3) -> Self{
        Self{
            // Treat the two points a and b as extrema for the bounding box, so we don't require a
            // particular minimum/maximum coordinate order.
            x : Interval::new_arg(a.e[0].min(b.e[0]), a.e[0].max(b.e[0])),
            y : Interval::new_arg(a.e[1].min(b.e[1]), a.e[0].max(b.e[1])),
            z : Interval::new_arg(a.e[2].min(b.e[2]), a.e[0].max(b.e[2])),
        }
    }

    pub fn axis(&self, n : i32) -> Interval{
        if n == 1 { return self.y;}
        if n == 2 { return self.z;}
        self.x
    }

    pub fn hit(&self, r : &mut Ray, mut ray_t : Interval) -> bool{
        for a in 0..3{
            // t_0 and t_1 rely on 1/ b_x (or y/z), so this takes care of this annoying case by simply registering a hit
            if r.direction().e[a] == 0.0 {
                continue;
            }

            let invD = 1.0 / r.direction().e[a];
            let orig = r.origin().e[a];

            let mut t0 = (self.axis(a as i32).min - orig) * invD;
            let mut t1 = (self.axis(a as i32).max - orig) * invD;

            if invD < 0.0{
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.min { ray_t.min = t0;}
            if t1 < ray_t.max { ray_t.max = t1;}

            if ray_t.max <= ray_t.min{
                return false;
            }
        }
        

        true
    }

    pub fn pad(&self) -> AABB{
        // Return an AABB that has no side narrower than some delta, padding if necessary.
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta { self.x} else {self.x.expand(delta)};
        let new_y = if self.y.size() >= delta { self.y} else {self.y.expand(delta)};
        let new_z = if self.z.size() >= delta { self.z} else {self.z.expand(delta)};

        AABB::new_arg(new_x, new_y, new_z)
    }
}