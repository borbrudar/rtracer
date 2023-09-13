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
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u*u*(3.0-2.0*u);
        v = v*v*(3.0-2.0*v);
        w = w*w*(3.0-2.0*w);
        

        let i = p.x() as i32;       
        let j = p.y() as i32;
        let k = p.z() as i32;

        let mut c : [[[f64;2];2];2] = [[[0.0;2];2];2] ;

        for di in 0..2{
            for dj in 0..2{
                for dk in 0..2{
                    c[di][dj][dk] = self.ranfloat[(self.perm_x[((i as usize)+di) & 255] ^ 
                        self.perm_y[((j as usize) + dj) & 255] ^ 
                        self.perm_z[((k as usize) + dk) & 255])  as usize];
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
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
        for i in (1..n).rev(){
            let target = random_int(0,i);
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }


    pub fn trilinear_interp(c :[[[f64;2];2];2], u : f64, v : f64, w : f64) -> f64{
        let mut accum = 0.0;
        for i in 0..2{
            for j in 0..2{
                for k in 0..2{
                    let fi = i as f64;
                    let fj = j as f64;
                    let fk = k as f64;
                    accum += (fi*u + (1.0-fi) * (1.0-u)) * (fj*v + (1.0-fj) * (1.0-v)) * (fk*w + (1.0-fk) * (1.0-w)) * c[i][j][k];
                }
            }
        }
        
        accum
    }
}