use std::fs::File;
use std::io::Read;

pub fn run(){
    println!("Day 1");

    let mut file = File::open("data/day1.txt").expect("Could not open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file.");

    let mut col0: Vec<i32> = Vec::new();
    let mut col1: Vec<i32> = Vec::new();

    for line in contents.lines(){
        let split_line: Vec<i32> = line.split_whitespace().map(|x|->i32{x.parse().unwrap()}).collect();
        col0.push(split_line[0]);
        col1.push(split_line[1]);
    }

    col0.sort();
    col1.sort();

    
    let mut distance_sum: i32 = 0;
    let mut similarity_sum: i32 = 0;
    for i in 0..col0.len(){
        // Part 1
        distance_sum = distance_sum + (col0[i] - col1[i]).abs();

        // Part 2
        similarity_sum = similarity_sum + col0[i] * (col1.iter().filter(|&x| *x == col0[i]).count() as i32);
    }

    println!("Part 1: {:?}", distance_sum);
    println!("Part 2: {:?}", similarity_sum);
}