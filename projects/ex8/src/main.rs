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

    let _text =
"Add Sally to Engineering\n\
Add Amir to Sales\n\
Add Foo to Production\n\
Add Fee to Support\n\
Add Foo to R&D\n\
Add Fee to R&D\n";
    let text: Vec<&str> = _text.split("\n").collect();
//    println!("text={:?}", text);
    let mut comp = HashMap::new();
    for t in text {
//        println!("t={:?}", t);
        if t.len() == 0 { continue }
        let mut i = t.split_whitespace();
//        println!("i={:?}", i);
        let verb = i.next().expect("verb not found");
        let emp = i.next().expect("emp not found");
        let dir = i.next().expect("dir not found");
        let dept = i.next().expect("dept not found");
        if verb != "Add" && dir != "to" {
            continue;
        }
        let d = comp.entry(dept).or_insert(vec![]);
        d.push(emp);
    }
    let mut depts: Vec<&str> = comp.keys().cloned().collect();
    depts.sort();
    println!("depts={:#?}", depts);
    for d in depts {
        println!("All people in department '{}':", d);
        let mut emps = comp.get(d).cloned().expect("dept not found");
        emps.sort();
        println!("{:?}", emps);
    }
    let mut emps: HashMap<&str, Vec<&str>> = HashMap::new();
    for (k,v) in comp {
        for emp in v {
            let n = emps.entry(emp).or_insert(Vec::new());
            n.push(k);
        }
    }
    let mut semps: Vec<&str> = emps.keys().cloned().collect();
    semps.sort();
    for e in semps {
        println!("All depts for employee '{}':", e);
        let mut depts = emps.get(e).cloned().expect("emp not found");
        depts.sort();
        println!("{:?}", depts);
    }
}
