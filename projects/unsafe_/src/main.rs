fn main() {
    let (num, _num2) = (4u8, 10u8);
    //let num = [5u8, 10];

    let r1 = &num as *const u8;
    //let r1 = &num[0] as *const u8;
    //    let r2 = &mut num as *mut u8;
    let main = main as *const u8;
    //    println!("r1={:#?} r2={:#?}", r1, r2);
    println!("r1={:#?}", r1);
    //    println!("ptr={:#?}", &num.as_mut_ptr());
    for o in 0..16 {
        print!(" {:#02x}", unsafe { *r1.offset(o) });
    }
    println!("");
    println!("main={:#?}", main);
    for o in 0..16 {
        print!(" {:#02x}", unsafe { *main.offset(o) });
    }
    println!("");

    let i = 42;
    let pi = &i as *const i32;
    unsafe {
        let pi = pi as *mut i32;
        *pi = 24;
    }
    assert_eq!(24, i);
}
