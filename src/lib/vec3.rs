
use std::ops::{Add, AddAssign, Mul, MulAssign,  Sub, SubAssign,  Div, DivAssign};
use num_traits::identities::Zero; 

/// A generic Number super trait which places bounds on the types allowed to be used in a Vec3
pub trait Number :  Zero + Add + Mul<Output = Self> + Sub<Output = Self> + Div<Output = Self> + AddAssign + MulAssign + SubAssign + DivAssign + Copy { 

}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
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

// A blank implementation of the abstract Number Super Trait that really just enforces 
// that types that are in our Vec3 are some sort of numeric which we expect. This way 
// our vector only ever has types like uint8 which can be limiting in some cases.
macro_rules! number_impl {
    ($($t:ty)*) => ($(
        impl Number for $t { }
    )*)
}
number_impl!{u8 u16 u32 u64 usize f32 f64  i8 i16 i32 i64}

// The operator assing for addition and subtraction are very similiar but really 
// I wanted to try writing a macro
macro_rules! ops_assign_impl {
    ($operand: tt, $trait_name : ident, $fn_name : ident ) => {
        impl<T: Number> $trait_name for Vec3<T> { 
            fn $fn_name(&mut self, other : Self) {
                self.x $operand other.x; 
                self.y $operand other.y; 
                self.z $operand other.z; 
            }
        }
    };
}   

ops_assign_impl!(+=, AddAssign, add_assign); 
ops_assign_impl!(-=, SubAssign, sub_assign); 

// public methods for Vector
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

    /// Take the vector dot product of vectors u and v respectively
    pub fn dot(u : &Self, v: &Self) -> T { 
       (u.x * v.x) + (u.y * v.y) + (u.z * v.z) 
    }


    pub fn len_squared(&self) -> T{ 
        self.x * self.x + self.y * self.y + self.z * self.z 
    }

    pub fn cross(u : &Self, v : &Self) -> Self { 
        Vec3 { 
            x: u.y * v.z - u.z * v.y, 
            y: u.z * v.x - u.x * v.z, 
            z : u.x * v.y - u.y * v.x
        }
    }
}

// Rust does not carry a square root function that works on all types so types must be converted to floats 
// before returning a value. This means that one needs to convert between T and f64 but Rust does not allow 
// the implicit Conversion between T regardless on what limitations one puts on T so instead we make a macro and 
// implement the methods on various types we may use.
macro_rules! vec_norm_impl {
    ($t:ty) => {
        impl Vec3<$t> { 
            
            /// Take the unit length of the vector. Do note that the inner values of the vectors are converted to 64-bit floats for 
            /// taking the square root of the squared norm of the vector. This may lead to innacuracies in specific cases where floating point values are not used.
            pub fn len(&self) -> $t { 
                ((self.len_squared() as f64).sqrt()) as $t
            }

            // Find the vector with norm of 1 in the same direction as our current vector. Note that this may be innacurate when using vectors that are 
            // not double or single precision floats since taking the square root to find the norm of a vector relies on converision to and from a double 
            // floating point number. 
            pub fn unit_vec(&self) -> Self { 
                let len = self.len(); 
                Vec3 { 
                    x: self.x/len, 
                    y: self.y/len, 
                    z: self.z/len
                }
            }
        }
    };
}

vec_norm_impl!(f32);
vec_norm_impl!(f64);
vec_norm_impl!(u8);
vec_norm_impl!(u16);
vec_norm_impl!(u32);
vec_norm_impl!(u64);


// vector addition
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

// divide a vector by a scalar
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

// scale a vector by a scalar
impl<T: Number> Mul<T> for Vec3<T> { 
    type Output = Self;

    fn mul(self, scalar: T) ->Self::Output {
        Self { 
            x : self.x * scalar, 
            y: self.y * scalar, 
            z : self.z * scalar
        }
    }
}

impl<T: Number> MulAssign<T> for Vec3<T> { 
    
fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar; 
        self.y *= scalar; 
        self.z *= scalar; 
    }
}

impl <T: Number> Div<T> for Vec3<T> { 
    type Output = Self; 

    fn div(self, scalar: T) -> Self::Output { 

        Self { 
            x : self.x / scalar, 
            y : self.y / scalar, 
            z : self.z / scalar,  
        }
    }
}

impl<T: Number> DivAssign<T> for Vec3<T> { 
    fn div_assign(&mut self, scalar: T) { 
        self.x /= scalar;
        self.y /= scalar; 
        self.z /= scalar; 
    }
}

#[cfg(test)]
mod test{ 

    use super::*; 

    #[test]
    fn add_test() {
        let mut first = Vec3::<u8>::new(1, 2, 3); 
        let second = Vec3::<u8>::new(4, 5, 6);

        let expected = Vec3::<u8>::new(5, 7, 9);

        let add_out = first + second; 

        assert_eq!(add_out, expected); 
        first += second; 
        assert_eq!(first, expected); 

    }

    #[test]
    fn sub_test() {
        let mut first = Vec3::<u8>::new(1, 2, 3); 
        let second = first.clone(); 
        let expected = Vec3::<u8>::zero(); 

        let sub_out = first - second; 
        assert_eq!(sub_out, expected); 
        first -= second; 
        assert_eq!(first, expected)
    }

    #[test]
    fn mul_test() {
        let mut first = Vec3::<u8>::new(1, 2, 3); 
        let scalar : u8= 2; 
        let expected = Vec3::<u8>::new(2, 4, 6);
        
        let out =   first*scalar; 
        assert_eq!(out, expected); 
        first *= scalar; 
        assert_eq!(first, expected); 
    }

    #[test]
    fn div_test() {
        let mut first = Vec3::<u8>::new(2, 4, 6);
        let expected = Vec3::<u8>::new(1, 2, 3); 
        let scalar : u8= 2; 
        
        let out = first/scalar; 
        assert_eq!(out, expected); 
        first /= scalar; 
        assert_eq!(first, expected); 

    }

    

    #[test]
    fn dot_test() {
        unimplemented!();
    }

    #[test]
    fn cross_test() {
        unimplemented!();
    }

}