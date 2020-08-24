use std::collections::HashMap;
fn main() {
    let l = vec![2, 300, 1, 30, 2, 301, 21, 56, 1, 7, 19, 300, 2, 300, 56, 12, 300];
    let mut sum = 0;
    let mut n = 0;
    for i in &l {
        sum += i;
        n += 1;
    }
    println!("l={:?}", l);
    let mut sl = l;
    sl.sort();
    println!("sl={:?}", sl);
    println!("n={}", n);
    let mean = sum / n;
    let mut median = 0;
    let mut mode = 0;
    let mut m = 0;
    for i in &sl {
        if m >= n / 2 {
            median = *i;
            break;
        }
        m += 1;
    }
    let mut stats = HashMap::new();
    let mut nmode = 0;
    for i in &sl {
        let count = stats.entry(i).or_insert(0);
        *count += 1;
        if *count > nmode {
            nmode = *count;
            mode = *i;
        }
    }
    println!("stats={:?}", stats);
    println!("sum={}", sum);
    println!("mean={}", mean);
    println!("median={}", median);
    println!("mode={}", mode);

//    let s = "apple";//apple-hay
    let s = "first";//irst-fay
    println!("s={:?}", s);
    let vow = ["a", "e", "i", "o", "u", "y"];
    let mut pigl = String::new();
    if vow.contains(&&s[0..1]) {
        println!("starts with vowel");
        pigl = format!("{}-hay", s);
    } else {
        println!("starts with consonant");
        pigl = format!("{}-{}ay", &s[1..], &s[0..1]);
    }
    println!("pig latin={:?}", pigl);

    let text = "Add Sally to Engineering";
    let mut comp = HashMap::new();
    comp.insert("R&D", vec!["Foo", "Fee"]);
    for (k,v) in comp {
        println!("All people in department '{}':", k);
        for emp in v.sort() {
            println!("  {}", emp);
        }
    }
}
