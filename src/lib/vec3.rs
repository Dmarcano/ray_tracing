
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

type Color = Vec3<u8>;

type Point = Vec3<f64>;

impl<T : Number> Vec3<T> { 

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
    type Output = Self;

    fn add(self, other:Self) -> Self::Output {
        Self { 
            x : self.x + other.x, 
            y : self.y + other.y, 
            z : self.z + other.z
        }
    }
}

impl<T: Number + Sub<Output = T>> Sub for Vec3<T> { 
    type Output = Self; 

    fn sub(self, other : Self) -> Self::Output { 
        Self { 
            x : self.x - other.x, 
            y : self.y - other.y, 
            z : self.z - other.z
        }
    }
}

impl <T : Number> Mul for Vec3<T> { 
    type Output = Self; 

    
    fn mul(self, other: Self) ->Self::Output { 
        todo!() 
    }
}