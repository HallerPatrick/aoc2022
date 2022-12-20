use crate::utils::read_file_lines;
use std::process;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Crate {
    None,
    Some(String),
}

#[derive(Debug, Clone)]
struct CrateTowers {
    matrix: Vec<Vec<Crate>>,
}

impl CrateTowers {

    fn display(&self) {

        let rows = self.matrix.iter().rev().take(10).rev();

        for row in rows {
            println!("{:?}", row);
        }
    }


    fn from_lines(lines: Vec<String>) -> CrateTowers {
        let mut matrix = Vec::new();

        for line in lines {
            let mut spaces_iter = line.chars();

            let mut row = Vec::new();

            while let Some(item) = spaces_iter.next() {
                if item == ' ' {
                } else if item == '*' {
                    row.push(Crate::None);
                } else if item == '[' {
                    let character = spaces_iter.next(); // Character
                    spaces_iter.next(); // [
                    row.push(Crate::Some(character.unwrap().to_string()));
                }
            }
            
            if !row.is_empty() {
                matrix.push(row);
            }

        }

        let width = matrix.first().unwrap().len();
       
        let mut top_spaces = Vec::new();

        for _row in 0..100 {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Crate::None);
            }
            top_spaces.push(row);
        }
        // Add spacing rows on top
        //
        top_spaces.extend(matrix);

        CrateTowers { matrix: top_spaces }
    }

    fn apply_instruction(&mut self, instruction: (i32, i32, i32)) {

        let (amount, origin, target) = instruction;

        for i in 0..amount {
            self.move_crate(origin.try_into().unwrap(), target.try_into().unwrap());
        }
    }

    fn apply_instruction_order_retained(&mut self, instruction: (i32, i32, i32)) {
        let (amount, origin, target) = instruction;

        for i in 0..amount {
            self.move_crate(origin.try_into().unwrap(), target.try_into().unwrap());
        }
        self.rename_stack(target.try_into().unwrap(), amount);
    }

    fn rename_stack(&mut self, target: usize, amount: i32) {

        let mut current_stack = Vec::new();

        let mut current_amoun = amount;


        for row_i in 0..self.matrix.len() {
            if current_amoun > 0 {
                if self.matrix[row_i][target-1] != Crate::None {
                    current_stack.push(self.matrix[row_i][target-1].clone());
                    current_amoun -= 1;
                    self.matrix[row_i][target-1] = Crate::None;
                }
            }
        }

        for crate_ in current_stack {
            for row_i in 0..self.matrix.len() {
                let cell = &self.matrix[row_i][target-1];
                
                if cell != &Crate::None {
                    self.matrix[row_i-1][target-1] = crate_.clone();
                    break
                }

                if row_i == self.matrix.len() -1 {
                    self.matrix[row_i][target-1] = crate_.clone();
                    break
                }
            }
            
        }
    }


    fn move_crate(&mut self, origin: usize, target: usize) {
        
        let mut target_crate: Option<Crate> = None;
        
        // Get first not None crate
        for i in 0..self.matrix.len() {
            let cell = &self.matrix[i][origin-1];
            if cell != &Crate::None {
                // Found it, take it and replace with None
                target_crate = Some(cell.clone());
                self.matrix[i][origin-1] = Crate::None;
                break
            }
        }

        for row_i in 0..self.matrix.len() {
            let cell = &self.matrix[row_i][target-1];
            
            if cell != &Crate::None {
                self.matrix[row_i-1][target-1] = target_crate.unwrap().clone();
                break
            }

            if row_i == self.matrix.len() -1 {
                self.matrix[row_i][target-1] = target_crate.unwrap().clone();
                break
            }
        }
    }

    fn count_top_crates(&self) {

        let mut top_crates = Vec::new();


        for col in 0..self.matrix.first().unwrap().len() {
            for row in 0..self.matrix.len() {
                if self.matrix[row][col] != Crate::None {
                    top_crates.push(self.matrix[row][col].clone());
                    break;
                }
            }
        }

        println!("{:?}", top_crates);
    }
}

fn parse_towers(lines: Vec<String>) -> CrateTowers {
    // We modified input file, such that "empty" spaces are "*" for easier parsing
    CrateTowers::from_lines(lines)
}

fn parse_instructions(instructions: Vec<String>) -> Vec<(i32, i32, i32)> {
    let mut instrs = Vec::new();

    for instr in instructions {
        let mut instr_iter = instr.split(" ");

        instr_iter.next(); // Move
        let amount = instr_iter.next().unwrap().parse::<i32>().unwrap();
        instr_iter.next(); // from
        let origin = instr_iter.next().unwrap().parse::<i32>().unwrap();
        instr_iter.next(); // to
        let target = instr_iter.next().unwrap().parse::<i32>().unwrap();

        instrs.push((amount, origin, target))
    }

    instrs
}

fn split_vec(vec: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut first_group = Vec::new();
    let mut second_group = Vec::new();

    let mut first = true;

    for item in vec {
        if item == "" {
            first = false;
        } else if first {
            first_group.push(item);
        } else {
            second_group.push(item)
        }
    }

    (first_group, second_group)
}

fn part1(lines: Vec<String>) {
    let (towers, instructions_lines) = split_vec(lines);

    let mut tower = parse_towers(towers);
    let instructions = parse_instructions(instructions_lines);

    for instr in instructions {
        tower.apply_instruction(instr);
    }

    tower.count_top_crates();
}

fn part2(lines: Vec<String>) {
    let (towers, instructions_lines) = split_vec(lines);

    let mut tower = parse_towers(towers);
    let instructions = parse_instructions(instructions_lines);

    for instr in instructions {
        tower.apply_instruction_order_retained(instr);
    }

    tower.count_top_crates();
}

pub fn run() {
    let lines = read_file_lines("./src/day5/input.txt");

    part1(lines.clone());
    part2(lines);
}
