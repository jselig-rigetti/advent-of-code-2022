use std::{fmt::Display, str::FromStr};

struct TreeGrid(Vec<Vec<u8>>);

impl FromStr for TreeGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|c| c.to_string().parse::<u8>().expect("expected digit"))
                        .collect()
                })
                .collect(),
        ))
    }
}

impl TreeGrid {
    fn get(&self, row: usize, col: usize) -> &u8 {
        self.0.get(row).unwrap().get(col).unwrap()
    }

    fn num_rows(&self) -> usize {
        self.0.len()
    }

    fn num_cols(&self) -> usize {
        self.0.iter().map(|row| row.len()).max().unwrap()
    }
}

struct Visibility {
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            top: false,
            right: false,
            bottom: false,
            left: false,
        }
    }
}

impl Visibility {
    fn is_visible(&self) -> bool {
        self.top || self.right || self.bottom || self.left
    }
}

struct VisibilityGrid(Vec<Vec<Visibility>>);

impl Display for VisibilityGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tree| if tree.is_visible() { "T" } else { "-" })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", data)
    }
}

impl VisibilityGrid {
    fn from_tree_grid(tree_grid: &TreeGrid) -> Self {
        let mut visibility_grid = Self(
            tree_grid
                .0
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|_| Visibility::default())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        );

        let num_rows = tree_grid.num_rows();
        let num_cols = tree_grid.num_cols();

        for row in 0..num_rows {
            // left to right
            let mut highest_from_left: &u8 = &0;
            for col in 0..num_cols {
                let tree = tree_grid.get(row, col);
                if col == 0 || tree > highest_from_left {
                    highest_from_left = tree;
                    visibility_grid.set_left(row, col, true);
                }
            }

            // right to left
            let mut highest_from_right: &u8 = &0;
            for col in (0..num_cols).rev() {
                let tree = tree_grid.get(row, col);
                if col == num_cols - 1 || tree > highest_from_right {
                    highest_from_right = tree;
                    visibility_grid.set_right(row, col, true);
                }
            }
        }

        for col in 0..num_cols {
            // top to bottom
            let mut highest_from_top: &u8 = &0;
            for row in 0..num_rows {
                let tree = tree_grid.get(row, col);
                if row == 0 || tree > highest_from_top {
                    highest_from_top = tree;
                    visibility_grid.set_top(row, col, true);
                }
            }

            // bottom to top
            let mut highest_from_bottom: &u8 = &0;
            for row in (0..num_rows).rev() {
                let tree = tree_grid.get(row, col);
                if row == num_rows - 1 || tree > highest_from_bottom {
                    highest_from_bottom = tree;
                    visibility_grid.set_bottom(row, col, true);
                }
            }
        }

        visibility_grid
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Visibility {
        self.0.get_mut(row).unwrap().get_mut(col).unwrap()
    }

    fn set_top(&mut self, row: usize, col: usize, visibile: bool) {
        self.get_mut(row, col).top = visibile;
    }

    fn set_right(&mut self, row: usize, col: usize, visibile: bool) {
        self.get_mut(row, col).right = visibile;
    }

    fn set_bottom(&mut self, row: usize, col: usize, visibile: bool) {
        self.get_mut(row, col).bottom = visibile;
    }

    fn set_left(&mut self, row: usize, col: usize, visibile: bool) {
        self.get_mut(row, col).left = visibile;
    }

    fn count_all_visible(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|tree| tree.is_visible()))
            .flatten()
            .count()
    }
}

fn parse_input(input: &str) -> (TreeGrid, VisibilityGrid) {
    let tree_grid = TreeGrid::from_str(input).expect("expected parsed tree grid");
    let visibility_grid = VisibilityGrid::from_tree_grid(&tree_grid);

    (tree_grid, visibility_grid)
}

fn get_part_1_answer(visibility_grid: &VisibilityGrid) -> usize {
    visibility_grid.count_all_visible()
}

fn get_part_2_answer(tree_grid: &TreeGrid) -> usize {
    let num_rows = tree_grid.num_rows();
    let num_cols = tree_grid.num_cols();

    let mut max_score: usize = 0;

    for row in 0..num_rows {
        for col in 0..num_cols {
            let tree = tree_grid.get(row, col);

            let mut top_score = 0;
            let mut right_score = 0;
            let mut bottom_score = 0;
            let mut left_score = 0;

            // top
            let mut offset: usize = 1;
            while let Some(row) = row.checked_sub(offset) {
                top_score += 1;
                if tree_grid.get(row, col) >= tree {
                    break;
                }
                offset += 1;
            }

            // right
            let mut offset: usize = 1;
            while col + offset < num_cols {
                right_score += 1;
                if tree_grid.get(row, col + offset) >= tree {
                    break;
                }
                offset += 1;
            }

            // bottom
            let mut offset: usize = 1;
            while row + offset < num_cols {
                bottom_score += 1;
                if tree_grid.get(row + offset, col) >= tree {
                    break;
                }
                offset += 1;
            }

            // left
            let mut offset: usize = 1;
            while let Some(col) = col.checked_sub(offset) {
                left_score += 1;
                if tree_grid.get(row, col) >= tree {
                    break;
                }
                offset += 1;
            }

            let curr_score = top_score * right_score * bottom_score * left_score;
            if curr_score > max_score {
                max_score = curr_score;
            }
        }
    }

    max_score
}

pub(crate) fn solve(input: String) -> String {
    let (tree_grid, visibility_grid) = parse_input(&input);

    let part_1_answer = get_part_1_answer(&visibility_grid);
    let part_2_answer = get_part_2_answer(&tree_grid);

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
    30373
    25512
    65332
    33549
    35390";

    #[rstest(input, expected, case(INPUT, 21))]
    fn test_part_1(input: &str, expected: usize) {
        let (_, visibility_grid) = parse_input(input);
        assert_eq!(get_part_1_answer(&visibility_grid), expected);
    }

    #[rstest(input, expected, case(INPUT, 8))]
    fn test_part_2(input: &str, expected: usize) {
        let (tree_grid, _) = parse_input(input);
        assert_eq!(get_part_2_answer(&tree_grid), expected);
    }
}
