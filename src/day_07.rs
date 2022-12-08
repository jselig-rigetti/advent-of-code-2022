use std::{collections::HashMap, str::FromStr};

struct File {
    size: u32,
}

struct Dir {
    dirs: HashMap<String, Dir>,
    files: HashMap<String, File>,
}

impl Default for Dir {
    fn default() -> Self {
        Self {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

impl Dir {
    fn get_size(&self) -> u32 {
        let size_dirs: u32 = self.dirs.values().map(|dir| dir.get_size()).sum();
        let size_files: u32 = self.files.values().map(|file| file.size).sum();

        size_dirs + size_files
    }
}

#[derive(Debug)]
enum LsOutput {
    Dir { name: String },
    File { name: String, size: u32 },
}

impl LsOutput {
    fn add_to_dir(&self, dir: &mut Dir) {
        match self {
            Self::Dir { name } => {
                dir.dirs.insert(name.clone(), Dir::default());
            }
            Self::File { name, size } => {
                dir.files.insert(name.clone(), File { size: size.clone() });
            }
        }
    }
}

impl FromStr for LsOutput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let dir_or_size = parts.next().expect("expected dir or size");
        let name = parts.next().map(String::from).expect("expected name");

        Ok(match dir_or_size {
            "dir" => LsOutput::Dir { name },
            _ => LsOutput::File {
                name,
                size: dir_or_size.parse().expect("expected size is u32"),
            },
        })
    }
}

enum Cmd {
    Cd { name: String },
    Ls { output: Vec<LsOutput> },
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let mut cmd_and_arg = lines
            .next()
            .expect("expected first line is command")
            .split_whitespace();

        Ok(
            match cmd_and_arg.next().expect("cmd must have executable name") {
                "cd" => Cmd::Cd {
                    name: cmd_and_arg
                        .next()
                        .expect("cmd must have name arg")
                        .to_string(),
                },
                "ls" => Cmd::Ls {
                    output: lines
                        .map(|line| line.parse().expect("ls output must parse"))
                        .collect(),
                },
                cmd => panic!("unrecognized command: {}", cmd),
            },
        )
    }
}

fn parse_history(input: &str) -> Vec<Cmd> {
    input
        .split('$')
        .skip(1) // this should be `cd /`
        .map(Cmd::from_str)
        .map(|cmd| cmd.expect("expected cmd to parse"))
        .collect()
}

fn parse_cmds(cmds: Vec<Cmd>) -> Dir {
    let mut root = Dir::default();
    let mut path = vec![];

    for cmd in cmds {
        match cmd {
            Cmd::Cd { name } if name == ".." => {
                path.pop().expect("expected a parent dir");
            }
            Cmd::Cd { name } => {
                path.push(name);
            }
            Cmd::Ls { output } => {
                let cwd = path.iter().skip(1).fold(&mut root, |dir, name| {
                    dir.dirs.get_mut(name).expect("expected dir to exist")
                });
                output.iter().for_each(|line| line.add_to_dir(cwd));
            }
        }
    }

    root
}

fn get_sizes(name: String, dir: &Dir) -> Vec<(String, u32)> {
    let mut subdir_sizes: Vec<(String, u32)> = dir
        .dirs
        .iter()
        .map(|(name, dir)| get_sizes(name.to_owned(), dir))
        .flatten()
        .collect();

    subdir_sizes.insert(0, (name, dir.get_size()));

    subdir_sizes
}

fn get_dirs_sum_lte_threshold(dir: &Dir, threshold: u32) -> u32 {
    let lte_threshold = get_sizes("/".to_string(), dir)
        .into_iter()
        .filter_map(
            |(_, size)| {
                if size <= threshold {
                    Some(size)
                } else {
                    None
                }
            },
        );

    lte_threshold.sum()
}

fn parse_fs(input: &str) -> Dir {
    parse_cmds(parse_history(input))
}

fn get_part_1_answer(dir: &Dir) -> u32 {
    get_dirs_sum_lte_threshold(dir, 100_000)
}

fn get_part_2_answer(dir: &Dir) -> u32 {
    let total_fs_size: u32 = 70_000_000;
    let needs_fs_size: u32 = 30_000_000;

    let curr_fs_free = total_fs_size - dir.get_size();
    let clean_fs_size = needs_fs_size - curr_fs_free;

    get_sizes("/".to_string(), dir)
        .into_iter()
        .filter_map(|(_, size)| {
            if size >= clean_fs_size {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .expect("expected min freeable dir")
}

pub(crate) fn solve(input: String) -> String {
    let dir = parse_fs(&input);
    let part_1_answer = get_part_1_answer(&dir);
    let part_2_answer = get_part_2_answer(&dir);

    format!(
        r#"
Part 1: sum dirs under 100_000 threshold: {}
Part 2: smallest deletable dir to free size: {}
"#,
        part_1_answer, part_2_answer,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    static INPUT: &str = "$ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k";

    #[rstest(input, expected, case(INPUT, 95_437))]
    fn test_part_1(input: &str, expected: u32) {
        assert_eq!(get_part_1_answer(&parse_fs(input)), expected);
    }

    #[rstest(input, expected, case(INPUT, 24_933_642))]
    fn test_part_2(input: &str, expected: u32) {
        assert_eq!(get_part_2_answer(&parse_fs(input)), expected);
    }
}
