
use crate::utils::read_file_lines;

use std::collections::VecDeque;


fn part2(line: String) {
    let mut iter = line.chars();

    let mut current_stack: VecDeque<char> = VecDeque::with_capacity(4);
    
    let mut current_index = 14;

    for _ in 0..14 {
        current_stack.push_back(iter.next().unwrap());
    }

    while !are_values_unique(&current_stack) {
        current_stack.pop_front();
        current_stack.push_back(iter.next().unwrap());
        current_index += 1;
    }

    println!("Index: {}", current_index);
}

fn part1(line: String) {

    let mut iter = line.chars();

    let mut current_stack: VecDeque<char> = VecDeque::with_capacity(4);
    
    // Init first 4 chars
    current_stack.push_back(iter.next().unwrap());
    current_stack.push_back(iter.next().unwrap());
    current_stack.push_back(iter.next().unwrap());
    current_stack.push_back(iter.next().unwrap());

    let mut current_index = 4;

    while !are_values_unique(&current_stack) {
        current_stack.pop_front();
        current_stack.push_back(iter.next().unwrap());
        current_index += 1;
    }

    println!("Index: {}", current_index);

}

fn are_values_unique(values: &VecDeque<char>) -> bool {
    let mut set = std::collections::HashSet::new();
    for value in values {
        if set.contains(&value) {
            return false;
        }
        set.insert(value);
    }
    true
}


pub fn run() {

    let lines = read_file_lines("./src/day6/input.txt");

    let line = lines.first().unwrap().to_string();

    part1(line.clone());
    part2(line);
}
