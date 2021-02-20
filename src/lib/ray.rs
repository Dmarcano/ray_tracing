use crate::lib::vec3::{Vec3, Point};

/// A ray in 3d space who starts at an origin point in space
/// and traverses across a line in a specific direction in space
/// 
/// One can think of a ray as an equation P(T) = A + B*t
/// where A is the origin and B is the direction vector
pub struct Ray { 
    pub origin : Point, 
    pub direction : Point, 
}

impl Ray { 
    pub fn new(origin : Point, direction : Point) -> Self { 
        Ray { 
            origin, 
            direction 
        }
    }

    // given a value of t, gives where in space someone is should they follow they ray's direction 
    // from its origin to the value t
    pub fn at(&self, t : f64) -> Vec3<f64>{ 
        self.origin + (  self.direction * t )
    }
}