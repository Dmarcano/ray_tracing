
use std::ops::{Add, AddAssign, Mul, MulAssign,  Sub, SubAssign,  Div, DivAssign,  Neg};
use num_traits::identities::Zero; 

/// A generic Number which can be assigned, 
pub trait Number :  Zero + Add + Mul + Sub + Div + Neg + AddAssign + MulAssign + SubAssign + DivAssign{ 

}

pub struct Vec3<T> 
where T : Number
{ 
    x : T, 
    y : T, 
    z : T 
}


impl<T> Vec3<T> 
where T :Number
{ 

    pub fn new(x : T, y : T , z: T) -> Vec3<T> { 
        Vec3 { 
            x, 
            y, 
            z
        }
    }

    pub fn zero() -> Vec3<T> { 
        Vec3 { 
            x : T::zero(), 
            y: T::zero(), 
            z: T::zero()
        }
    }
}

impl<T : Number> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other:Self) -> Self::Output {
        Self { 
            x : self.x + other.x, 
            y : self.y + other.y, 
            z : self.z + other.z
        }
    }
}