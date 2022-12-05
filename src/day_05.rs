// https://adventofcode.com/2022/day/5

use std::{env, fmt::Display, fs, str::FromStr};

#[derive(Debug)]
pub(crate) struct Instruction {
    source: usize,
    target: usize,
    count: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [_, count, _, source, _, target] =
            s.split_ascii_whitespace().array_chunks().next().unwrap();
        Ok(Self {
            source: source.parse::<usize>().unwrap(),
            target: target.parse::<usize>().unwrap(),
            count: count.parse::<usize>().unwrap(),
        })
    }
}

#[derive(Debug)]
pub(crate) struct CrateStacks {
    data: Vec<Vec<char>>,
}

impl CrateStacks {
    pub(crate) fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, vec![]);
        Self { data }
    }

    fn push(&mut self, stack: usize, char: char) {
        self.data.get_mut(stack).unwrap().push(char);
    }

    fn prepend(&mut self, stack: usize, char: char) {
        self.data.get_mut(stack).unwrap().insert(0, char);
    }

    fn take(&mut self, stack: usize) -> char {
        self.data.get_mut(stack).unwrap().remove(0)
    }

    fn swap(&mut self, source: usize, target: usize) {
        let c = self.take(source);
        self.prepend(target, c);
    }
}

impl Display for CrateStacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stack in self.data.iter() {
            for char in stack {
                write!(f, "{} ", char)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

pub(crate) fn parse_file(file: &str) -> (CrateStacks, Vec<Instruction>) {
    let (head, tail) = file.split_once("\n\n").unwrap();
    let head = head
        .lines()
        .take_while(|&line| !line.is_empty())
        .collect::<Vec<_>>();
    let instructions = tail
        .lines()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    let (&crate_indices, crate_data) = head.split_last().unwrap();

    let num_crates = crate_indices
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .max()
        .unwrap();

    let mut crate_stacks = CrateStacks::new(num_crates);

    for &line in crate_data {
        let chunks = format!("{} ", line)
            .chars()
            .array_chunks()
            .enumerate()
            .map(|(col, [_, c, _, _])| (col, c))
            .filter(|(_, c)| !c.is_whitespace())
            .collect::<Vec<_>>();

        for (col, c) in chunks {
            crate_stacks.push(col, c);
        }
    }

    (crate_stacks, instructions)
}

pub(crate) fn apply_instructions(
    mut crate_stacks: CrateStacks,
    instructions: &Vec<Instruction>,
) -> CrateStacks {
    for &Instruction {
        source,
        target,
        count,
    } in instructions
    {
        for _ in 0..count {
            crate_stacks.swap(source - 1, target - 1);
        }
    }

    crate_stacks
}

pub(crate) fn solve() -> String {
    let input = env::args()
        .nth(1)
        .map(fs::read_to_string)
        .expect("failed to read input file");

    let (crate_stacks, instructions) = input
        .as_deref()
        .map(parse_file)
        .expect("failed to parse data");

    let finished_stacks = apply_instructions(crate_stacks, &instructions);

    let top_crates = finished_stacks
        .data
        .into_iter()
        .map(|stack| stack.get(0).unwrap_or(&' ').clone())
        .collect::<String>();

    format!(
        r#"
Part 1: top crates: {}
"#,
        top_crates,
    )
}
