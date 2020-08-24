#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        self.w * self.h
    }
    fn can_hold(&self, o: &Rect) -> bool {
        self.w >= o.w && self.h >= o.h
    }
    fn square(size: u32) -> Rect {
        Rect { w: size, h: size }
    }
}
fn main() {
    let rect1 = Rect { w: 30, h: 50 };
    let rect2 = Rect { w: 10, h: 40 };
    let rect3 = Rect { w: 60, h: 45 };
    let a = rect1.area();
    println!("area is {}", a);
    println!("rect1 is {:#?}", rect1);
    println!("can rect1 hold rect2? is {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? is {}", rect1.can_hold(&rect3));
}
