use std::{collections::HashMap, fs::File, io::Read};

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
    let mut value_map_right: HashMap<i32, u32> = HashMap::new();
    for value in right {
        match value_map_right.get_mut(&value) {
            Some(map_val) => {*map_val += value as u32;},
            None => {value_map_right.insert(value, value as u32);}
        };
    }

    let mut sim_score: u32 = 0;
    for value in left {
        sim_score += match value_map_right.get(&value) {
            Some(map_val) => map_val,
            None => &0
        }
    }

    println!("{sim_score}");
}
