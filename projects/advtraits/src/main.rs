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

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
struct Wrapper(Vec<i32>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0[0], self.0[1], self.0[2])
    }
}
impl OutlinePrint for Point {}
impl OutlinePrint for Vec3 {}
impl OutlinePrint for Wrapper {}

fn main() {
    let v1 = vec3!(1., 2., 3.);
    let v2 = vec3!(4., 5., 6.);
    let v = v1 + v2 + 3.14;
    println!("v={:?}", v);

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let p = Point { x: 34, y: 56 };
    p.outline_print();
    v.outline_print();

    let v2 = Wrapper(vec![10, 9, 8]);
    v2.outline_print();
}
