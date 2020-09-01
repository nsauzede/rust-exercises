/*
overloadable ops :
+ +=
- -=
/ /=
* *=

& &=
| |=
^ ^=
% %/

-, !

*/
use std::ops::Add;

#[derive(Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

fn main() {
    let v1 = vec3!(1., 2., 3.);
    let v2 = vec3!(4., 5., 6.);
    let v = v1 + v2 + 3.14;
    println!("v={:?}", v);
}
