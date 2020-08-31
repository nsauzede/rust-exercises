#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let p = Point { x: 0, y: 7 };

    //    let Point { x: a, y: b } = p;
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet={} inches={} x={} y={}", feet, inches, x, y);
    let ((feet, inches), p) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet={} inches={} p={:?}", feet, inches, p);
    let (t, p) = ((3, 10), Point { x: 3, y: -10 });
    println!("tuple={:?} point={:?}", t, p);

    let s = Some(String::from("Hello!"));

    if let Some(_s) = s.clone() {
        println!("found a string {:?}", s);
    }

    println!("{:?}", s);

    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { y, .. } => println!("y is {}", y),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (_, first, .., last, _) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n @ 10) => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    #[derive(Debug)]
    enum Message2 {
        Hello { id: i32 },
    }

    let msg = Message2::Hello { id: 15 };

    match msg {
        Message2::Hello { id } if id >= 13 && id <= 20 => println!("Found an id in Hrange: {}", id),
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in Lrange: {}", id_variable),
        Message2::Hello { id: 10..=12 } => {
            let Message2::Hello { id } = msg;
            println!("Found an id in another Mrange {:?}", id)
        }
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}
