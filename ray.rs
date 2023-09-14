use crate::rvec3::*;

pub struct Ray{
    orig : Point3,
    dir : Rvec3,
    time : f64,
}

impl Default for Ray{
    fn default()-> Self{
        Self::new()
    }
}

impl Ray{
   pub fn new() -> Self {
       Self{
           orig: Point3::new(),
           dir : Rvec3::new(),
           time : 0.0
        }
    }
    pub fn new_arg(_orig : Point3, _dir : Rvec3) -> Self{
        let mut drr = _dir;
        drr = Rvec3::unit_vector(&mut drr);
        Self{
            orig : _orig,
            dir : drr,
            time : 0.0
        }
    }
    pub fn new_time(_orig : Point3, _dir : Rvec3, tm : f64) -> Self{
        let mut drr = _dir;
        drr = Rvec3::unit_vector(&mut drr);
        Self{
            orig : _orig,
            dir : drr,
            time : tm
        }
    }

    pub fn origin(&mut self) -> Point3 { self.orig }
    pub fn direction(&mut self) -> Rvec3 { self.dir }
    pub fn time(&mut self) -> f64 {self.time}

    pub fn at(&mut self, t : f64) -> Point3{
        self.orig + t*self.dir
    }
} 
