
use std::ops::*;
// point3 is just an alias for vec3, but useful for geometric clarity in the code.
pub type Point3 = Rvec3;
use crate::utility::*;


#[derive(Copy,Clone)]
pub struct Rvec3{
    pub e : [f64;3],
}

impl Rvec3{
    pub fn new() -> Rvec3 {
        Rvec3{
            e : [0.0,0.0,0.0]
        }
    }
    pub fn new_arg(x : f64, y:f64, z : f64) -> Rvec3{
        Rvec3{
            e : [x,y,z]
        }
    }
    pub fn init(&mut self, e0 : f64, e1 : f64, e2 : f64) {
        self.e[0] = e0;
        self.e[1] = e1;
        self.e[2] = e2;
    }
    pub fn x(&mut self) -> f64{ self.e[0]}
    pub fn y(&mut self) -> f64{ self.e[1]}
    pub fn z(&mut self) -> f64{ self.e[2]}

    
    pub fn length_squared(&mut self) -> f64{
        self.e[0]*self.e[0] + self.e[1] *self.e[1] + self.e[2] *self.e[2]
    }

    pub fn length(&mut self) -> f64{
        self.length_squared().sqrt()
    }


    pub fn dot(&u : &Rvec3, &v : &Rvec3) -> f64{
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn unit_vector(u : &mut Rvec3) -> Rvec3{
        *u / u.length()
    }

    pub fn cross(&u : &Rvec3, &v : &Rvec3) -> Rvec3{
        Rvec3{
            e : [u.e[1] * v.e[2] - u.e[2] * v.e[1], 
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0]]
        }
    }

    pub fn random_vec() -> Rvec3 {
        Rvec3{e : [random_double(), random_double(), random_double()]}
    }

    pub fn random_vec_range(min : f64, max : f64) -> Rvec3{
        Rvec3{e : [random_range(min,max),random_range(min,max),random_range(min,max)]}
    }

    pub fn random_in_unit_sphere() -> Rvec3 {
        loop{
            let mut p = Rvec3::random_vec_range(-1.0,1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Rvec3{
        Rvec3::unit_vector(&mut Rvec3::random_in_unit_sphere())
    }

    pub fn random_on_hemisphere(normal : &Rvec3) -> Rvec3{
        let on_unit_sphere = Rvec3::random_unit_vector();
        if Rvec3::dot(&on_unit_sphere,normal) > 0.0{
            return on_unit_sphere;
        }
        -on_unit_sphere
    }

    pub fn near_zero(&mut self) -> bool{
        // Return true if the vector is close to zero in all dimensions.
        let s = 0.00000001; //1e-8
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }


    pub fn reflect(v : Rvec3, n : Rvec3) -> Rvec3{
        v - 2.0*Rvec3::dot(&v,&n)*n
    } 
}

impl Default for Rvec3{
    fn default() -> Self{
        Self::new()
    }
}


impl Neg for Rvec3{
    type Output = Self;
    
    fn neg(mut self) -> Self::Output{
        self.e[0] = -self.e[0];
        self.e[1] = -self.e[1];
        self.e[2] = -self.e[2];
        self
    }
}

impl Index<usize> for Rvec3 {
    type Output = f64;

    fn index(&self, i : usize) -> &Self::Output {
        &self.e[i]
    }
}

impl IndexMut<usize> for Rvec3 {
    fn index_mut(&mut self, i : usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}

impl AddAssign for Rvec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e:[self.e[0] + other.e[0],self.e[1]+other.e[1],self.e[2]+other.e[2]],
        };
    }
}

impl MulAssign<Rvec3> for Rvec3 {
    fn mul_assign(&mut self, rhs: Rvec3) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl DivAssign<f64> for Rvec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] *= 1.0/rhs;
        self.e[1] *= 1.0/rhs;
        self.e[2] *= 1.0/rhs;
    }
}

// Vector Utility Functions


impl std::fmt::Display for Rvec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(e0: {}, e1: {}, e2: {})", self.e[0], self.e[1],self.e[2])
    }
}


impl Add for Rvec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [self.e[0] + other.e[0],self.e[1] + other.e[1], self.e[2] + other.e[2]],
        }
    }
}


impl Sub for Rvec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]
        }
    }
}

impl Mul for Rvec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]]
        }
    }
}

impl Mul<f64> for Rvec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self { 
            e: [self.e[0] * rhs,self.e[1] * rhs, self.e[2]*rhs] 
        }
    }
}

impl Mul<Rvec3> for f64 {
    type Output = Rvec3;

    fn mul(self, rhs: Rvec3) -> Rvec3 {
        Rvec3 { 
            e: [rhs.e[0] * self,rhs.e[1] * self, rhs.e[2]*self] 
        }
    }
}

impl Div<f64> for Rvec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }
        self * (1.0/rhs)
    }
}