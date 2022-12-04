#[derive(Debug, PartialEq)]
enum Outcome {
    _Unspecified,
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        return match self {
            Self::_Unspecified => panic!("Invalid outcome"),
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        };
    }

    fn parse_move(c: char) -> Outcome {
        return match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Invalid outcome"),
        };
    }
}

#[derive(Debug, PartialEq)]
enum RPS {
    _Unspecified,
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_score(score: u32) -> RPS {
        return match score {
            1 => Self::Rock,
            2 => Self::Paper,
            3 => Self::Scissors,
            _ => panic!("Invalid score"),
        };
    }

    fn score(&self) -> u32 {
        return match self {
            Self::_Unspecified => panic!("Unspecified does not give a score"),
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        };
    }

    fn move_for_outcome(&self, outcome: Outcome) -> RPS {
        let offset: u32 = match outcome {
            Outcome::_Unspecified => panic!("Invalid outcome"),
            Outcome::Lose => 2,
            Outcome::Draw => 0,
            Outcome::Win => 1,
        };

        let self_score = self.score() - 1;
        let score_offset = (self_score + offset) % 3;
        return Self::from_score(score_offset + 1);
    }

    fn outcome(&self, other: Self) -> Outcome {
        let self_score = (self.score() % 3) as i32;
        let them_score = (other.score() % 3) as i32;
        let diff = (self_score - them_score).rem_euclid(3);

        return match diff {
            2 => Outcome::Lose,
            0 => Outcome::Draw,
            1 => Outcome::Win,
            _ => panic!("Unreachable"),
        };
    }

    fn parse_oppoent_move(c: char) -> Self {
        return match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            any => panic!("invalid letter {}", any),
        }
    }

    fn parse_self_move(c: char) -> Self {
        return match c {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            any => panic!("invalid letter {}", any),
        }
    }

    fn score_line_1(line: String) -> u32 {
        let moves: Vec<&str> = line.split(" ").collect();
        assert!(moves.len() == 2);
        let them = moves[0].chars().nth(0usize).unwrap();
        let us = moves[1].chars().nth(0usize).unwrap();

        let them = Self::parse_oppoent_move(them);
        let us = Self::parse_self_move(us);

        return us.score() + us.outcome(them).score();
    }

    fn score_line_2(line: String) -> u32 {
        let moves: Vec<&str> = line.split(" ").collect();
        let them = moves[0].chars().nth(0usize).unwrap();
        let outcome = Outcome::parse_move(moves[1].chars().nth(0usize).unwrap());

        let them = Self::parse_oppoent_move(them);
        let us = them.move_for_outcome(outcome);

        return us.score() + us.outcome(them).score();
    }
}

fn test_contests() {
    assert_eq!(RPS::Rock.outcome(RPS::Paper), Outcome::Lose);
    assert_eq!(RPS::Rock.outcome(RPS::Rock), Outcome::Draw);
    assert_eq!(RPS::Rock.outcome(RPS::Scissors), Outcome::Win);

    assert_eq!(RPS::Paper.outcome(RPS::Scissors), Outcome::Lose);
    assert_eq!(RPS::Paper.outcome(RPS::Paper), Outcome::Draw);
    assert_eq!(RPS::Paper.outcome(RPS::Rock), Outcome::Win);

    assert_eq!(RPS::Scissors.outcome(RPS::Rock), Outcome::Lose);
    assert_eq!(RPS::Scissors.outcome(RPS::Scissors), Outcome::Draw);
    assert_eq!(RPS::Scissors.outcome(RPS::Paper), Outcome::Win);
}

fn test_beaters() {
    assert_eq!(RPS::Rock.move_for_outcome(Outcome::Win), RPS::Paper);
    assert_eq!(RPS::Rock.move_for_outcome(Outcome::Lose), RPS::Scissors);
    assert_eq!(RPS::Rock.move_for_outcome(Outcome::Draw), RPS::Rock);

    assert_eq!(RPS::Paper.move_for_outcome(Outcome::Win), RPS::Scissors);
    assert_eq!(RPS::Paper.move_for_outcome(Outcome::Lose), RPS::Rock);
    assert_eq!(RPS::Paper.move_for_outcome(Outcome::Draw), RPS::Paper);

    assert_eq!(RPS::Scissors.move_for_outcome(Outcome::Win), RPS::Rock);
    assert_eq!(RPS::Scissors.move_for_outcome(Outcome::Lose), RPS::Paper);
    assert_eq!(RPS::Scissors.move_for_outcome(Outcome::Draw), RPS::Scissors);
}

fn main() {
    test_contests();
    test_beaters();

    let lines = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let score_a: u32 = lines
        .iter()
        .map(|line| line.to_string())
        .map(RPS::score_line_1)
        .sum();

    let score_b: u32 = lines
        .iter()
        .map(|line| line.to_string())
        .map(RPS::score_line_2)
        .sum();

    println!("score_a: {}", score_a);
    println!("score_b: {}", score_b);
}
