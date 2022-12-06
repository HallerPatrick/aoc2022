use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum HandShape {
    Rock,
    Paper,
    Scissor
}


impl HandShape {
    fn from_string(string_value: &str) -> HandShape {
        match string_value {
            "A" | "X" => HandShape::Rock,
            "B" | "Y" => HandShape::Paper,
            "C" | "Z" => HandShape::Scissor,
            _ => panic!("Symbol {} not known!", string_value)

        }
    }

    fn value(&self) -> i16 {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissor => 3,
        }
    }

    fn play(&self, other: HandShape) -> i16 {
        if *self == other {
            3
        } else if *self == HandShape::Rock && other == HandShape::Scissor {
            6
        } else if *self == HandShape::Paper && other == HandShape::Rock {
            6
        } else if *self == HandShape::Scissor && other == HandShape::Paper {
            6
        } else {
            0
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum GameOutcome {
    Win,
    Lose,
    Draw
}

impl GameOutcome {
    fn from_handshape(hs_value: HandShape) -> GameOutcome {
        match hs_value {
            HandShape::Rock => GameOutcome::Lose,
            HandShape::Paper => GameOutcome::Draw,
            HandShape::Scissor => GameOutcome::Win,
        }
    }
}

fn convert_lines_to_games(lines: Vec<&str>) -> Vec<(HandShape, HandShape)> {

    let mut games = Vec::new();

    for line in lines {
        let value_vec = line.trim().split(" ").collect::<Vec<&str>>();
        if value_vec.len() == 1 {
            continue;
        }
        let mut values = value_vec.iter();
        let first_pick = HandShape::from_string(values.next().unwrap());
        let second_pick = HandShape::from_string(values.next().unwrap());
        games.push((first_pick, second_pick))
    }
    
    games
}

fn part1(game: Vec<(HandShape, HandShape)>) {

    let mut scores = 0;
    for round in game {
        let shape_value = round.1.value();
        let score = round.1.play(round.0);
        scores += score + shape_value;
    }
    

    println!("Final Score: {}", scores);

}

fn win(handshape: &HandShape) -> HandShape {
    match handshape {
        HandShape::Rock => HandShape::Paper,
        HandShape::Paper => HandShape::Scissor,
        HandShape::Scissor => HandShape::Rock,
    }
}

fn lose(handshape: &HandShape) -> HandShape {
    match handshape {
        HandShape::Rock => HandShape::Scissor,
        HandShape::Paper => HandShape::Rock,
        HandShape::Scissor => HandShape::Paper,
    }
}

fn part2(game: Vec<(HandShape, HandShape)>) {

    let mut scores = 0;

    for round in game {
        let hand_shape = round.0;
        let outcome = GameOutcome::from_handshape(round.1);

        let required_hd = match outcome {
            GameOutcome::Win => win(&hand_shape),
            GameOutcome::Lose => lose(&hand_shape),
            GameOutcome::Draw => hand_shape.clone()
        };

        scores += required_hd.play(hand_shape) + required_hd.value();

    }

    println!("Final score is: {}", scores);
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Input.txt not found");
    let lines = file.split("\n").collect::<Vec<&str>>();
    let game = convert_lines_to_games(lines);
    part1(game.clone());
    part2(game);
}
