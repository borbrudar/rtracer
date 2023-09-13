use crate::utility::*;
use std::vec::*;
use crate::rvec3::*;

pub struct Perlin{
    ranfloat : Vec<f64>,
    perm_x : Vec<i32>,
    perm_y : Vec<i32>,
    perm_z : Vec<i32>,
}

impl Perlin {
    const point_count : i32 = 256;
  
    pub fn new() -> Self{
        let mut tmp : Vec<f64> = Vec::new();
        for i in 0..Perlin::point_count {
            tmp.push(random_double());
        }
        
        Self {
            ranfloat : tmp,
            perm_x : Perlin::perlin_generate_perm(),
            perm_y : Perlin::perlin_generate_perm(),
            perm_z : Perlin::perlin_generate_perm(),                
        }
    }

    pub fn noise(&self, p : &mut Point3) -> f64{
        let i = (4*p.x() as i32) & 255;       
        let j = (4*p.y() as i32) & 255;
        let k = (4*p.z() as i32) & 255;

        self.ranfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize])  as usize]
    }


    pub fn perlin_generate_perm() -> Vec<i32> {
        let mut p : Vec<i32> = Vec::new();
        for i in 0..Perlin::point_count{
            p.push(i);
        }
        Perlin::permute(&mut p,Perlin::point_count);

        p
    }

    pub fn permute(p : &mut Vec<i32>, n : i32) {
        for i in (n-1)..0{
            let target = random_int(0,i);
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }

    }
}