#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

type Scal = f32;
#[derive(Debug)]
struct V3(Scal, Scal, Scal);
macro_rules! v3 {
    ($x:expr, $y:expr, $z:expr) => {
        V3($x as Scal, $y as Scal, $z as Scal)
    };
}

fn main() {
    let v = vec![1, 2, 3];
    let vbis = vec![1, 2, 3];
    assert_eq!(v, vbis);
    println!("{:?}", v);

    let v3 = v3!(123, 3.14f64, 6.66);
    println!("{:?}", v3);
}
