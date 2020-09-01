use mytools;

type Scal = f32;
//type Scal = f64;
//type Scal = i32;

#[derive(Debug)]
struct Vec3(Scal, Scal, Scal);

macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3($x as Scal, $y as Scal, $z as Scal)
    };
}

fn main() {
    let v = vec3!(3.14f32, 1.1e3f64, 666);
    println!("{:?} {}", v, mytools::type_of(&v));
    println!("{} {}", v.0, mytools::type_of(&v.0));
}
