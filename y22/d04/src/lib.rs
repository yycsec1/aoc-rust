use std::ops::RangeInclusive;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::IResult;


pub fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u8>> {
    let (input, start) = complete::u8(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = complete::u8(input)?;
    Ok((input, start..=end))
}

pub fn parse_ranges(input: &str) -> IResult<&str, (RangeInclusive<u8>, RangeInclusive<u8>)> {
    let (input, range1) = parse_range(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, range2) = parse_range(input)?;
    Ok((input, (range1, range2)))
}

pub fn part_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| parse_ranges(line).unwrap().1)
        .filter(|(range1, range2)| {
            range1.clone().into_iter().all(|num| range2.contains(&num)) || range2.clone().into_iter().all(|num| range1.contains(&num))
        }).count()
}

pub fn part_b(input: &str) -> usize {
    input
        .lines()
        .map(|line| parse_ranges(line).unwrap().1)
        .filter(|(range1, range2)| {
            range1.clone().into_iter().any(|num| range2.contains(&num)) || range2.clone().into_iter().any(|num| range1.contains(&num))
        }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_sample() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_a(input), 2);
    }

    #[test]
    fn test_part_a() {
        let input = include_str!("../input.txt");
        assert_eq!(part_a(input), 562);
    }

    #[test]
    fn test_part_b_sample() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_b(input), 4);
    }

    #[test]
    fn test_part_b() {
        let input = include_str!("../input.txt");
        assert_eq!(part_b(input), 924);
    }
}
