use crate::rvec3::*;

pub struct Ray {
    orig: Point3,
    dir: Rvec3,
}

impl Default for Ray {
    fn default() -> Self {
        Self::new()
    }
}

impl Ray {
    pub fn new() -> Self {
        Self {
            orig: Point3::new(),
            dir: Rvec3::new(),
        }
    }
    pub fn new_arg(_orig: Point3, _dir: Rvec3) -> Self {
        Self {
            orig: _orig,
            dir: _dir,
        }
    }

    pub fn origin(&mut self) -> Point3 {
        self.orig
    }
    pub fn direction(&mut self) -> Rvec3 {
        self.dir
    }

    pub fn at(&mut self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
