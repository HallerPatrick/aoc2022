use crate::utils::read_file_lines;

#[derive(Debug, Copy, Clone)]
struct Range {
    begin: i32,
    end: i32,
}

impl Range {
    pub fn from_string(str: String) -> Range {
        let mut split_string = str.split("-");
        let (start, end) = (split_string.next().unwrap(), split_string.next().unwrap());

        Range {
            begin: start.parse::<i32>().unwrap(),
            end: end.parse::<i32>().unwrap(),
        }
    }

    fn contains(self, other: &Range) -> bool {
        if self.begin >= other.begin && self.end <= other.end {
            return true;
        }

        false
    }

    fn overlap(self, other: &Range) -> bool {
        if ((other.begin <= self.begin) && (self.begin <= other.end) ) ||
            ((other.begin <= self.end) && (self.end <= other.end)) {
                return true;
            }
        false
    }
}

fn part1(lines: Vec<String>) {

    let mut contained_pairs = 0;

    for line in lines {
        let mut line_splits = line.split(",");

        let (left_str, right_str) = (line_splits.next().unwrap(), line_splits.next().unwrap());

        let (left, right) = (
            Range::from_string(left_str.to_string()),
            Range::from_string(right_str.to_string()),
        );

        if left.contains(&right) || right.contains(&left) {
            contained_pairs += 1;
            println!("This pairs: {:?} -> {:?}", left, right);
        }

    }

    println!("Found contained pairs: {}", contained_pairs);
}

fn part2(lines: Vec<String>) {

    let mut overlapping_pairs = 0;

    for line in lines {
        let mut line_splits = line.split(",");

        let (left_str, right_str) = (line_splits.next().unwrap(), line_splits.next().unwrap());

        let (left, right) = (
            Range::from_string(left_str.to_string()),
            Range::from_string(right_str.to_string()),
        );

        if left.overlap(&right) || right.overlap(&left) {
            overlapping_pairs += 1;
            println!("This pairs: {:?} -> {:?}", left, right);
        }

    }

    println!("Found overlapping pairs: {}", overlapping_pairs);

}

pub fn run() {
    let lines = read_file_lines("./src/day4/input.txt");
    part1(lines.clone());
    part2(lines);
}
