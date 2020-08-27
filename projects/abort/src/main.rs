use std::fs::File;
use std::io::ErrorKind;
fn main() {
    /*
    panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
    */
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("just created the file");
                    fc
                }
                Err(e) => panic!("can't create file: {:?}", e),
            },
            other_error => panic!("can't open file: {:?}", other_error),
        },
    };
}
