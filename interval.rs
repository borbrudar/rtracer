use crate::utility::*;

#[derive(Clone, Copy)]
pub struct Interval{
    pub min : f64,
    pub max : f64
}

impl Interval{
    // returns empty interval
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

    pub fn new_intervals(a : Interval, b : Interval) -> Self{
        Self { 
            min: a.min.min(b.min), 
            max: a.max.max(b.max),
        }
    }
  
    pub fn contains(&mut self, x : f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&mut self, x : f64) -> bool {
        self.min < x && x < self.max      
    }
    
    pub fn clamp(&mut self,x : f64) -> f64{
        if x < self.min {return self.min;}
        if x > self.max {return self.max;}
        x
    }

    pub fn size(&self) -> f64{
        self.max-self.min
    }

    pub fn expand(&self,delta : f64) -> Interval{
        let padding = delta/2.0;
        Interval { min: self.min-padding, max: self.max+padding }
    }

    const empty : Interval = Interval{min : INFINITY, max : -INFINITY};
    const universe : Interval = Interval{min : -INFINITY, max : INFINITY};
}