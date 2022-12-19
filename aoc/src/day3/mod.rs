
use crate::utils::read_file_lines;

use array_tool::vec::Intersect;

fn split_string_in_half(s: &str) -> (String, String) {
    let chars = s.chars();
    let mid = chars.clone().count() / 2;
    let first_half = chars.clone().take(mid);
    let second_half = chars.skip(mid);
    (first_half.collect(), second_half.collect())
}

fn get_priority(c: char) -> i32 {
    if c.is_lowercase() {
        (c as u8 - 'a' as u8 + 1) as i32
    } else if c.is_uppercase() {
        (c as u8 - 'A' as u8 + 27) as i32
    } else {
        // Return 0 for any non-alphabetic characters
        0
    }
}


fn part1(lines: Vec<String>) {

    let mut total_sum = 0;
    for backpack in lines {
        let (left, right ) = split_string_in_half(&backpack);
        let left_ps = left.chars().map(|c| get_priority(c)).collect::<Vec<i32>>();
        let right_ps = right.chars().map(|c| get_priority(c)).collect::<Vec<i32>>();
        
        let intersect: Vec<i32> = left_ps.intersect(right_ps);

        let matching = intersect.first().unwrap();

        total_sum += matching;
    }

    println!("Total sum: {}", total_sum);
}

fn part2(lines: Vec<String>) {

    let mut total_sum: i32 = 0;

    let mut iter = lines.iter().into_iter();

    while let (Some(first), Some(second), Some(third)) = (iter.next(), iter.next(), iter.next()) {
        let (f, s, t) = (
            first.chars().map(|c| get_priority(c)).collect::<Vec<i32>>(),
            second.chars().map(|c| get_priority(c)).collect::<Vec<i32>>(),
            third.chars().map(|c| get_priority(c)).collect::<Vec<i32>>()
        );

        let intersect = f.intersect(s.intersect(t));

        let matching = intersect.first().unwrap();

        total_sum += matching;
    }

    println!("Total sum: {}", total_sum);
}


pub fn run() {
    let lines = read_file_lines("./src/day3/input.txt");

    part1(lines.clone());
    part2(lines);
}
