use std::fs::File;
use std::io::Read;

fn is_report_safe(report: &Vec<i32>) -> bool { // Return: is_safe, and index_of_failure
        
    let mut report_safe: bool = true;

    let mut direction = 0; // 0=None, 1=Increasing, -1=Decreasing
    for i in 0..(report.len() - 1) {
        let diff = report[i] - report[i + 1];

        if diff.abs() > 3 || diff == 0{
            report_safe = false;
            break;
        }

        // diff * direction will only be negative if the signs of the factors are different, i.e. wrong.
        if direction != 0 && diff * direction < 0{
            report_safe = false;
            break;
        }
        
        // Is sequence increasing or decreasing?
        if diff > 0 {
            direction = 1;
        } else if diff < 0 {
            direction = -1;
        }
    }

    report_safe
} 

fn is_dampened_report_safe(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut dampened_vec = report.clone();
        dampened_vec.remove(i);
        if is_report_safe(&dampened_vec) {
            return true;
        }
    }
    false
}

pub fn run() {
    println!("Day 2");

    let mut file = File::open("data/day2.txt").expect("Could not open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file.");

    let mut num_safe_reports: i32 = 0;
    let mut num_safe_dampened_reports: i32 = 0;
    for report_str in contents.lines() {
        let report: Vec<i32> = report_str.split_whitespace().map(|x|->i32{x.parse().unwrap()}).collect();
        let report_safe = is_report_safe(&report);
        match report_safe {
            true => {
                num_safe_reports = num_safe_reports + (report_safe as i32);
                num_safe_dampened_reports = num_safe_dampened_reports + (report_safe as i32);
            },
            false => {
                num_safe_dampened_reports = num_safe_dampened_reports + (is_dampened_report_safe(&report) as i32);
            }
        }
    }

    println!("Part 1: {:?}", num_safe_reports);
    println!("Part 2: {:?}", num_safe_dampened_reports);
}