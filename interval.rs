use crate::utility::*;


pub struct Interval{
    pub min : f64,
    pub max : f64
}

impl Interval{
    pub fn new() -> Self{
        Self{
            min : INFINITY,
            max : -INFINITY
        }
    }  
    pub fn new_arg(mn : f64, mx : f64) -> Self{
        Self{
            min : mn,
            max : mx
        }
    }
  
    pub fn contains(&mut self, x : f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&mut self, x : f64) -> bool {
        self.min < x && x < self.max      
    }
  
    const empty : Interval = Interval{min : INFINITY, max : -INFINITY};
    const universe : Interval = Interval{min : -INFINITY, max : INFINITY};
}