use std::collections::HashMap;
use hashmap::hello;
fn main() {
//    println!("hello");
    hello();
    let s = "hello world".to_string();
    println!("s={:?}", s);
    let v = vec!(1, 2, 3, 4, 5);
    println!("v={:?}", v);
//    let t1 = "blue";
    let t1 = "blue".to_string();
//    let t2 = "yellow";
    let t2 = "yellow".to_string();
    let teams = vec![t1, t2];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("hm={:#?}", scores);
//    println!("t1={} t2={}", t1, t2);
    let n = "blue".to_string();
    println!("blue={:?}", scores.get(&n).expect("ouch"));
    scores.entry(String::from("blue")).or_insert(100);
    for (k, v) in scores {
        println!("{}: {}", k, v);
    }
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("text={:#?} stats={:?}", text, map);

}
