fn to_num(c: Option<char>) -> i8 {
    match c {
        Some('A') | Some('X') => 0,
        Some('B') | Some('Y') => 1,
        Some('C') | Some('Z') => 2,
        _ => unreachable!("unexpected character"),
    }
}

fn get_score_a(c: (i8, i8)) -> usize {
    let ans: usize = match c.0 - c.1 {
        -1 | 2 => 6,
        -2 | 1 => 0,
        0 => 3,
        _ => unreachable!("unexpected character"),
    };
    ans + c.1 as usize + 1
}

fn get_score_b(c: (i8, i8)) -> usize {
    let ans: usize = match c {
        (0, 0) | (2, 1) | (1, 2) => 3,
        (1, 0) | (0, 1) | (2, 2) => 1,
        (2, 0) | (0, 2) | (1, 1) => 2,
        _ => unreachable!("unexpected character"),
    };
    ans + c.1 as usize * 3
}

pub fn part_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| (to_num(line.chars().next()), to_num(line.chars().nth(2))))
        .map(get_score_a)
        .sum()
}

pub fn part_b(input: &str) -> usize {
    input
        .lines()
        .map(|line| (to_num(line.chars().next()), to_num(line.chars().nth(2))))
        .map(get_score_b)
        .sum()
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part_a_sample() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_a(input), 15);
    }

    #[test]
    fn test_part_a() {
        let input = include_str!("../input.txt");
        assert_eq!(part_a(input), 8392);
    }

    #[test]
    fn test_part_b_sample() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_b(input), 12);
    }

    #[test]
    fn test_part_b() {
        let input = include_str!("../input.txt");
        assert_eq!(part_b(input), 10116);
    }
}
