struct Turn {
    opponent: Shape,
    player: Shape,
}

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Turn {
    fn parse(s: &str) -> Self {
        let parts = s.split(" ").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);

        Turn {
            opponent: Shape::parse(parts[0]),
            player: Shape::parse(parts[1]),
        }
    }

    fn player_won(&self) -> bool {
        self.player.beats(&self.opponent)
    }

    fn points(&self) -> u32 {
        use Shape::*;
        let shape_points = match self.player {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        let outcome = if self.player == self.opponent {
            3
        } else if self.player_won() {
            6
        } else {
            0
        };
        shape_points + outcome
    }
}

impl Shape {
    fn parse(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid shape"),
        }
    }

    fn beats_shape(&self) -> Self {
        use Shape::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper 
        }
    }

    fn loses_against(&self) -> Self {
        use Shape::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }
    }

    fn beats(&self, other: &Shape) -> bool {
        use Shape::*;
        match self {
            Rock => *other == Scissors,
            Paper => *other == Rock,
            Scissors => *other == Paper,
        }
    }
}

fn new_rules(turn: &Turn) -> u32 {
    use Shape::*;
    let new_shape = match turn.player {
        Rock => turn.opponent.beats_shape(),
        Paper => turn.opponent,
        Scissors => turn.opponent.loses_against()
    };
    Turn {
        opponent: turn.opponent,
        player : new_shape
    }.points()
}

fn main() {
    let turns :Vec<Turn> = include_str!("input").lines().map(Turn::parse).collect();

    println!("Turns: {:?}", turns.len());
    let points : u32 = turns.iter().map(|turn| Turn::points(&turn)).sum();
    println!("Total points: {:?}", points);
    let points : u32 = turns.iter().map(|turn| new_rules(&turn)).sum();
    println!("Points with new rules: {:?}", points)
}
