enum Instruction {
    NOOP,
    ADD(i32)
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if s == "noop" {
            Self::NOOP
        } else {
            let parts = s.split(" ").collect::<Vec<_>>();
            assert_eq!(parts.len(), 2);
            assert_eq!(parts[0], "addx");
            Self::ADD(parts[1].parse().expect("Invalid number format"))
        }
    }
}

fn simulate_cpu(instructions: &Vec<Instruction>) -> Vec<i32> {
    let mut signal_values : Vec<i32> = vec![];
    let mut cycle = 0;

    let mut x = 1;

    let mut inc_cycle = |x: i32| {
        if (x-1..=x+1).contains(&(cycle % 40)) {
            print!("#");
        } else {
            print!(".");
        }

        cycle += 1;
        if cycle == 20 || (cycle - 20) % 40 == 0 {
            signal_values.push(x);
        }

        if cycle % 40 == 0 {
            println!();
        }
    };

    for instruction in instructions {
        match instruction {
            Instruction::NOOP => inc_cycle(x),
            Instruction::ADD(arg) => {
                inc_cycle(x);
                inc_cycle(x);
                x += arg;
            },
        }
    }

    signal_values
}


fn main() {
    let instructions = include_str!("input").lines().map(Instruction::from).collect::<Vec<_>>();
    let signal_strengths = simulate_cpu(&instructions);

    let mut cycle = 20;
    let mut sum = 0;

    for signal in signal_strengths {
        sum += cycle * signal;
        cycle += 40;
    }

    println!("Sum: {}", sum);
}
