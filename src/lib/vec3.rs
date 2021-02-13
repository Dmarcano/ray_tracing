
use std::ops::{Add, AddAssign, Mul, MulAssign,  Sub, SubAssign,  Div, DivAssign};
use num_traits::identities::Zero; 

/// A generic Number super trait which places bounds on the types allowed to be used in a Vec3
pub trait Number :  Zero + Add + Mul<Output = Self> + Sub<Output = Self> + Div<Output = Self> + AddAssign + MulAssign + SubAssign + DivAssign{ 

}

// A blank implementation of 
macro_rules! number_impl {
    ($($t:ty)*) => ($(
        impl Number for $t { }
    )*)
}

number_impl!{u8 u16 u32 u64 usize f32 f64  i8 i16 i32 i64}

/// A struct that represents some vector which has 3Dimensional quantities. 
pub struct Vec3<T> 
where T : Number
{ 
    pub x : T, 
    pub y : T, 
    pub z : T 
}

/// A vector that holds RGB values of up to 255
pub type Color = Vec3<u8>;

pub type Point = Vec3<f64>;

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

impl<T: Number> Sub for Vec3<T> { 
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
        Self { 
            x : self.x * other.x, 
            y : self.y * other.y, 
            z : self.z * other.z
        }
    }
}

impl <T: Number> Div for Vec3<T> { 
    type Output = Self; 

    fn div(self, other: Self) -> Self::Output { 

        Self { 
            x : self.x / other.x, 
            y : self.y / other.y, 
            z : self.z / other.z,  
        }
    }
}