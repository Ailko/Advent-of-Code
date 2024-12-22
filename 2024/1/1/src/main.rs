use std::{fs::File, io::Read, iter::zip};

fn main() {
    let mut buf = "".to_string();
    File::open("../input.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in buf.split("\n") {
        if line.len() == 0 {continue;}
        let values: (&str, &str) = match line.split_once("   ") {
            Some(value) => value,
            None => panic!("??????")
        };
        left.push(values.0.parse().unwrap());
        right.push(values.1.parse().unwrap());
    }
    left.sort();
    right.sort();
    let mut distance_tot = 0;
    for pair in zip(left, right) {
        distance_tot += (pair.0 - pair.1).abs();
    }
    println!("{distance_tot}");
}
