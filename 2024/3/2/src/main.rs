use std::{collections::VecDeque, fs::File, io::Read};
use regex::Regex;

fn main() {
    let mut memory = "".to_string();
    File::open("../input.txt")
        .unwrap()
        .read_to_string(&mut memory)
        .unwrap();
    let mut do_indices: VecDeque<usize> = Regex::new(r"do\(\)")
        .unwrap()
        .find_iter(&memory)
        .map(|cap| cap.start())
        .collect();
    let mut dont_indices: VecDeque<usize> = Regex::new(r"don't\(\)")
        .unwrap()
        .find_iter(&memory)
        .map(|cap| cap.start())
        .collect();
    let mut ranges: Vec<(usize, usize)> = vec![(0, memory.len())];
    while do_indices.len() > 0 && dont_indices.len() > 0 {
        if do_indices[0] < dont_indices[0] {
            let do_index  = do_indices.pop_front().unwrap(); 
            if ranges.last().unwrap().1 > do_index {
                continue;
            }
            ranges.push((do_index.clone(), memory.len()));
            continue;
        }
        let dont_index = dont_indices.pop_front().unwrap();
        if ranges.last().unwrap().1 < dont_index {
            continue;
        }
        ranges.last_mut().unwrap().1 = dont_index;
    }
    if do_indices.len() > 0 && do_indices[0] > ranges.last().unwrap().1 {
        ranges.push((do_indices[0], memory.len()));
    }
    if dont_indices.len() > 0 && dont_indices[0] < ranges.last().unwrap().1 {
        ranges.last_mut().unwrap().1 = dont_indices[0];
    }
    let mul_regex = Regex::new(r"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();
    let mut total: u32 = 0;
    for range in ranges {
        for capture in mul_regex.captures_iter(&memory[range.0..range.1]) {
            let num1 = capture.name("num1").unwrap().as_str().parse::<u32>().unwrap();
            let num2 = capture.name("num2").unwrap().as_str().parse::<u32>().unwrap();
            total += num1 * num2;
        }
    }
    println!("{total}");
}
