use std::str::FromStr;

enum Operation {
    Wait,
    Add(i32),
}

enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn into_operations(&self) -> Vec<Operation> {
        match self {
            Self::Noop => vec![Operation::Wait],
            Self::Addx(v) => vec![Operation::Wait, Operation::Add(v.clone())],
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace();
        let instruction = match parts.next().expect("expected operation name") {
            "noop" => Self::Noop,
            "addx" => Self::Addx(
                parts
                    .next()
                    .expect("expected addx value")
                    .parse::<i32>()
                    .unwrap(),
            ),
            raw => panic!("invalid instruction: {}", raw),
        };

        Ok(instruction)
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect()
}

fn execute_with_midcycle_sampler<F: FnMut(usize, &i32)>(
    instructions: &Vec<Instruction>,
    mut sampler: F,
) {
    let mut cycle: usize = 0;
    let mut mem = 1;

    for instruction in instructions {
        for operation in instruction.into_operations() {
            cycle += 1;
            sampler(cycle, &mem);
            match operation {
                Operation::Wait => {}
                Operation::Add(v) => mem += v,
            }
        }
    }
}

fn solve_part_1(instructions: &Vec<Instruction>) -> i32 {
    let mut total: i32 = 0;

    execute_with_midcycle_sampler(instructions, |cycle, value| {
        if cycle >= 20 && cycle <= 220 && (cycle - 20) % 40 == 0 {
            let signal_strength = (cycle as i32) * value;
            total += signal_strength;
        }
    });

    total
}

fn solve_part_2(instructions: &Vec<Instruction>) -> String {
    let mut screen = String::new();

    execute_with_midcycle_sampler(instructions, |cycle, value| {
        let x = (cycle - 1) % 40;

        if x == 0 {
            screen.push('\n');
        }

        let pixel = if (x as i32 - value).abs() <= 1 {
            '#'
        } else {
            ' ' // easier to read than '.'
        };

        screen.push(pixel);
    });

    screen
}

pub(crate) fn solve(input: String) -> String {
    let instructions = parse_input(&input);

    let part_1_answer = solve_part_1(&instructions);
    let part_2_answer = solve_part_2(&instructions);

    format!(
        r#"
Part 1: {}
Part 2: {}
"#,
        part_1_answer, part_2_answer,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    static INPUT: &str = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
    ";

    #[rstest(input, expected, case(INPUT, 13_140))]
    fn test_part_1(input: &str, expected: i32) {
        let instructions = parse_input(input);
        assert_eq!(solve_part_1(&instructions), expected);
    }
}
