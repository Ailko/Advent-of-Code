use std::{
    fs::File,
    io::{BufRead, BufReader}
};

fn main() {
    let file = File::open("../input.txt").unwrap();
    let mut safe: u32 = 0;
    for line in BufReader::new(file).lines() {
        match line {
            Ok(unparsed_report) => {
                if unparsed_report.len() == 0 {
                    continue;
                }
                let report: Vec<i32> = unparsed_report
                    .split(" ")
                    .map(|v| v.parse().unwrap())
                    .collect();
                let differences = report
                    .windows(2)
                    .map(|a| a[0] - a[1]);
                if !(differences.clone().all(|v| v < 0) || differences.clone().all(|v| v > 0)) {
                    continue;
                }
                if !differences.clone().all(|v| 0 < v.abs() && v.abs() < 4) {
                    continue;
                }
                safe += 1;
            },
            Err(e) => panic!("{e}")
        }
    };
    println!("{safe}");
}
