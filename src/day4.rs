use std::fs::File;
use std::io::Read;

fn get_horizontal_slice(data: &mut Vec<Vec<char>>, x_pos: usize, y_pos: usize) -> &str {
    return "MMMSXXMASM";
}

fn get_vertical_slice(data: &mut Vec<Vec<char>>, x_pos: usize, y_pos: usize) -> &str {
    return "MMMSXXMASM";
} 

fn get_diagonal_slice(data: &mut Vec<Vec<char>>, x_pos: usize, y_pos: usize) -> &str {
    return "MMMSXXMASM";
} 

fn check_position(data: &mut Vec<Vec<char>>, x_pos: usize, y_pos: usize) -> i32 {
    println!("Checking X: {:?}, Y: {:?}", x_pos, y_pos);

    let mut number_of_occurences: i32 = 0;

    {
        let horizontal_slice = get_horizontal_slice(data, x_pos, y_pos);
        if horizontal_slice.contains("XMAS") | horizontal_slice.contains("SAMX") {
            number_of_occurences = number_of_occurences + 1;
        } 
    } 

    {
        let vertical_slice = get_vertical_slice(data, x_pos, y_pos);
        if vertical_slice.contains("XMAS") | vertical_slice.contains("SAMX") {
            number_of_occurences = number_of_occurences + 1;
        } 
    }

    {
        let diagonal_slice = get_diagonal_slice(data, x_pos, y_pos);
        if diagonal_slice.contains("XMAS") | diagonal_slice.contains("SAMX") {
            number_of_occurences = number_of_occurences + 1;
        } 
    }

    return number_of_occurences;
} 

pub fn run(){
    println!("Day 4");

    let mut file = File::open("data/day4.txt").expect("Could not open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file.");

    let mut data: Vec<Vec<char>> = Vec::new();

    for line in contents.lines(){
        data.push(line.chars().collect());
    }

    let x_size = data[0].len();
    let y_size = data.len();

    println!("X-Size: {:?}, Y-Size: {:?}", x_size, y_size);

    let mut number_of_occurrences: i32 = 0;

    for x in 0..x_size {
        for y in 0..y_size {
            let matches = check_position(&mut data, x, y);
            number_of_occurrences = number_of_occurrences + matches;
        } 
    }

    println!("Part 1: {:?}", number_of_occurrences);
}