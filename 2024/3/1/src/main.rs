use std::{fs::File, io::Read};
use regex::Regex;

fn main() {
    let mut memory = "".to_string();
    File::open("../input.txt")
        .unwrap()
        .read_to_string(&mut memory)
        .unwrap();
    let mut total: u32 = 0;
    for capture in Regex::new(r#"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)"#).unwrap().captures_iter(&memory) {
        let num1 = capture.name("num1").unwrap().as_str().parse::<u32>().unwrap();
        let num2 = capture.name("num2").unwrap().as_str().parse::<u32>().unwrap();
        total += num1 * num2;
    }
    println!("{total}");
}
