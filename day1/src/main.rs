

use std::fs;

fn main() {
    println!("Solution Part1:");
    part1();

    println!("Solution Part2:");
    part2();
}

fn calories_per_elf(lines: Vec<&str>) -> Vec<i32> {

    let mut c_p_e = Vec::new();

    let mut tmp_calories = 0;

    for line in lines {
        match line.parse::<i32>() {
            Ok(calorie) => {
                tmp_calories += calorie;
            },
            Err(_) => {
                c_p_e.push(tmp_calories);
                tmp_calories = 0;
            }
        }
    }

    c_p_e
}

fn part1() {

    let file = fs::read_to_string("input.txt").expect("Input.txt not found");

    let lines = file.split("\n").collect::<Vec<&str>>();

    let calories_per_elf_vec = calories_per_elf(lines);
    
    let mut max_calories = 0;
    

    for (i, item) in calories_per_elf_vec.iter().enumerate() {
        if item > &max_calories {
            max_calories = *item;
        }
    }

    println!("Total Calories: {}", max_calories);
}

fn part2() {

    let file = fs::read_to_string("input.txt").expect("Input.txt not found");

    let lines = file.split("\n").collect::<Vec<&str>>();

    let mut calories_per_elf_vec = calories_per_elf(lines);
    calories_per_elf_vec.sort();
    
    println!("{:?}", calories_per_elf_vec.iter().rev().take(3).sum::<i32>());
}
