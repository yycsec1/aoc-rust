pub fn part_a(input: &str) -> usize {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|chunks| chunks
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        )
        .max()
        .unwrap()

}

pub fn part_b(input: &str) -> usize {
    let mut calories: Vec<usize> = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|chunks| chunks
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .sum::<usize>()
        )
        .collect();
    calories.sort_by(|a,b| b.cmp(a));
    calories.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part_a_sample() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_a(input), 24000);
    }

    #[test]
    fn test_part_a() {
        let input = include_str!("../input.txt");
        assert_eq!(part_a(input), 72478);
    }

    #[test]
    fn test_part_b_sample() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_b(input), 45000);
    }

    #[test]
    fn test_part_b() {
        let input = include_str!("../input.txt");
        assert_eq!(part_b(input), 210367);
    }
}
