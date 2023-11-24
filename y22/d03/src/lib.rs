use itertools::Itertools;

pub fn part_a(input: &str) -> usize {
    input.lines().map(common_item_a).map(priority).sum()
}

pub fn part_b(input: &str) -> usize {
    input.lines().tuples::<(&str, &str, &str)>().map(common_item_b).map(priority).sum()
}

pub fn common_item_a(rucksack: &str) -> char {
    let (comp_a, comp_b) = rucksack.split_at(rucksack.len()/2);
    comp_a.chars().filter(|c| comp_b.contains(*c)).next().unwrap()
}

pub fn common_item_b((rucksack_a, rucksack_b, rucksack_c): (&str, &str, &str)) -> char {
    rucksack_a.chars().filter(|c| rucksack_b.contains(*c)).filter(|c| rucksack_c.contains(*c)).next().unwrap()
}

pub fn priority(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - 'a' as usize + 1,
        'A'..='Z' => item as usize - 'A' as usize + 27,
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part_a_sample() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_a(input), 157);
    }

    #[test]
    fn test_part_a() {
        let input = include_str!("../input.txt");
        assert_eq!(part_a(input), 7737);
    }

    #[test]
    fn test_part_b_sample() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_b(input), 70);
    }

    #[test]
    fn test_part_b() {
        let input = include_str!("../input.txt");
        assert_eq!(part_b(input), 2697);
    }
}
