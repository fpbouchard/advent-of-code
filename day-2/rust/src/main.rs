use std::str::FromStr;

#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn base_score(&self) -> usize {
        match *self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

impl FromStr for Play {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> usize {
        match *self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn reverse_score(&self) -> usize {
        match *self {
            Outcome::Loss => 6,
            Outcome::Draw => 3,
            Outcome::Win => 0,
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent_play: Play,
    player_play: Play,
}

impl Round {
    fn score(&self) -> RoundScore {
        let player_outcome = match (&self.player_play, &self.opponent_play) {
            (Play::Rock, Play::Rock) => Outcome::Draw,
            (Play::Rock, Play::Paper) => Outcome::Loss,
            (Play::Rock, Play::Scissors) => Outcome::Win,
            (Play::Paper, Play::Rock) => Outcome::Win,
            (Play::Paper, Play::Paper) => Outcome::Draw,
            (Play::Paper, Play::Scissors) => Outcome::Loss,
            (Play::Scissors, Play::Rock) => Outcome::Loss,
            (Play::Scissors, Play::Paper) => Outcome::Win,
            (Play::Scissors, Play::Scissors) => Outcome::Draw,
        };
        RoundScore {
            player_score: self.player_play.base_score() + player_outcome.score(),
            opponent_score: self.opponent_play.base_score() + player_outcome.reverse_score(),
        }
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, player) = s.split_once(' ').unwrap();
        return Ok(Round {
            opponent_play: Play::from_str(opponent).unwrap(),
            player_play: Play::from_str(player).unwrap(),
        });
    }
}

#[derive(Debug)]
struct RoundScore {
    opponent_score: usize,
    player_score: usize,
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn round_scores(&self) -> Vec<RoundScore> {
        self.rounds.iter().map(|round| round.score()).collect()
    }

    fn player_score(&self) -> usize {
        self.round_scores().iter().map(|r| r.player_score).sum()
    }

    fn opponent_score(&self) -> usize {
        self.round_scores().iter().map(|r| r.opponent_score).sum()
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rounds = s.lines().fold(vec![], |mut acc, line| {
            acc.push(Round::from_str(line).unwrap());
            acc
        });

        return Ok(Game { rounds });
    }
}

fn load_data() -> String {
    return std::fs::read_to_string("./input.txt").unwrap();
}

fn main() {
    let game = Game::from_str(&load_data()).unwrap();
    println!("{:?}", game.player_score());
    println!("{:?}", game.opponent_score());
}
