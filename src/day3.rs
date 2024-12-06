use std::fs::File;
use std::io::Read;
use regex::Regex;

pub fn run(){
    println!("Day 3");

    let mut file = File::open("data/day3.txt").expect("Could not open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file.");

    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();

    let matches: Vec<&str> = regex.find_iter(&contents).map(|m| m.as_str()).collect();

    let mut accumulated_muls: i32 = 0;
    let mut accumulated_enabled_muls: i32 = 0;
    let mut enabled: bool = true;
    for mul_str in &matches {
        if *mul_str == "do()" {
            enabled = true;
            continue;
        }

        if *mul_str == "don't()" {
            enabled = false;
            continue;
        }

        // Remove first 4 characters "mul(", and last character ")"
        let sliced_mul_str = &mul_str[4..mul_str.len() - 1];

        let num_vec: Vec<&str> = sliced_mul_str.split(",").collect();
        accumulated_muls = accumulated_muls + (num_vec[0].parse::<i32>().unwrap() * num_vec[1].parse::<i32>().unwrap());

        if enabled {
            accumulated_enabled_muls = accumulated_enabled_muls + (num_vec[0].parse::<i32>().unwrap() * num_vec[1].parse::<i32>().unwrap());
        }
    }


    println!("Part 1: {:?}", accumulated_muls);
    println!("Part 2: {:?}", accumulated_enabled_muls);
}