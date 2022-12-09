use std::fs;

#[derive(Debug, Copy, Clone)]
enum Ply {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum Result {
    Win,
    Draw,
    Loss,
}

impl Ply {
    fn shape_score(self) -> i32 {
        match self {
            Ply::Rock => 1,
            Ply::Paper => 2,
            Ply::Scissors => 3,
        }
    }

    fn wins_against(self) -> Ply {
        match self {
            Ply::Rock => Ply::Scissors,
            Ply::Paper => Ply::Rock,
            Ply::Scissors => Ply::Paper,
        }
    }

    fn draws_against(self) -> Ply {
        self
    }

    fn loses_against(self) -> Ply {
        match self {
            Ply::Rock => Ply::Paper,
            Ply::Paper => Ply::Scissors,
            Ply::Scissors => Ply::Rock,
        }
    }
    fn beats(self, other: &Ply) -> Result {
        match (self, other) {
            (Ply::Rock, Ply::Paper) => Result::Loss,
            (Ply::Rock, Ply::Rock) => Result::Draw,
            (Ply::Rock, Ply::Scissors) => Result::Win,

            (Ply::Paper, Ply::Paper) => Result::Draw,
            (Ply::Paper, Ply::Rock) => Result::Win,
            (Ply::Paper, Ply::Scissors) => Result::Loss,

            (Ply::Scissors, Ply::Paper) => Result::Win,
            (Ply::Scissors, Ply::Rock) => Result::Loss,
            (Ply::Scissors, Ply::Scissors) => Result::Draw,
        }
    }

    fn score_against(self, other: &Ply) -> i32 {
        match self.beats(other) {
            Result::Win => self.shape_score() + 6,
            Result::Draw => self.shape_score() + 3,
            Result::Loss => self.shape_score(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = fs::read_to_string("./input").unwrap();
    if file.ends_with('\n') {
        file.pop();
    }
    let strategy: Vec<&str> = file.split('\n').collect();

    let total_score = strategy.iter().fold((0, 0), |scores, round| {
        let (left, right) = (round.chars().next().unwrap(), round.chars().nth(2).unwrap());
        let attack = match left {
            'A' => Ply::Rock,
            'B' => Ply::Paper,
            'C' => Ply::Scissors,
            // should never land here with AoC controlled inputs
            _ => panic!(),
        };
        let part1_response = match right {
            'X' => Ply::Rock,
            'Y' => Ply::Paper,
            'Z' => Ply::Scissors,
            // should never land here with AoC controlled inputs
            _ => panic!(),
        };
        let part2_response = match right {
            'X' => attack.wins_against(),
            'Y' => attack.draws_against(),
            'Z' => attack.loses_against(),
            // should never land here with AoC controlled inputs
            _ => panic!(),
        };

        (
            scores.0 + part1_response.score_against(&attack),
            scores.1 + part2_response.score_against(&attack),
        )
    });

    println!("x - {}", total_score.0);
    println!("y - {}", total_score.1);

    Ok(())
}
