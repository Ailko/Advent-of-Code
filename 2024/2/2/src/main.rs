use std::{
    fs::File,
    io::{BufRead, BufReader}
};

fn is_safe(report: &Vec<i32>) -> bool {
    let differences: Vec<i32> = report.clone()
                    .windows(2)
                    .map(|a| a[0] - a[1])
                    .collect();
    if !differences.clone().into_iter().all(|v| 0 < v.abs() && v.abs() < 4) {
        return false;
    }
    return differences.clone().into_iter().all(|v| v < 0) || differences.clone().into_iter().all(|v| v > 0)
}

fn main() {
    let file = File::open("../input.txt").unwrap();
    let mut safe: u32 = 0;
    'lines: for line in BufReader::new(file).lines() {
        match line {
            Ok(unparsed_report) => {
                if unparsed_report.len() == 0 {
                    continue;
                }
                let report: Vec<i32> = unparsed_report
                    .split(" ")
                    .map(|v| v.parse().unwrap())
                    .collect();
                if is_safe(&report) {
                    safe += 1;
                    continue;
                }
                for i in 0..report.len() {
                    let mut temp_report = report.clone();
                    temp_report.remove(i);
                    if is_safe(&temp_report) {
                        safe += 1;
                        continue 'lines; 
                    }
                }
            },
            Err(e) => panic!("{e}")
        }
    };
    println!("{safe}");
}
