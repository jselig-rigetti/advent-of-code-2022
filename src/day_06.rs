use std::collections::HashSet;

fn get_start_packet_index(input: &str, window_size: usize) -> usize {
    input
        .chars()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(window_size)
        .find(|slice| window_size == slice.iter().map(|(_, v)| v).collect::<HashSet<_>>().len())
        .unwrap()
        .last()
        .unwrap()
        .0
}

fn get_part_1_answer(input: &str) -> usize {
    get_start_packet_index(input, 4) + 1
}

fn get_part_2_answer(input: &str) -> usize {
    get_start_packet_index(input, 14) + 1
}

pub(crate) fn solve(input: String) -> String {
    let part_1_answer = get_part_1_answer(&input);
    let part_2_answer = get_part_2_answer(&input);

    format!(
        r#"
Part 1: packet start index: {}
Part 2: packet start index: {}
"#,
        part_1_answer, part_2_answer,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest(
        input,
        expected,
        case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        case("nppdvjthqldpwncqszvftbrmjlhg", 6),
        case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)
    )]
    fn test_part_1(input: &str, expected: usize) {
        assert_eq!(get_start_packet_index(input, 4) + 1, expected);
    }
}
