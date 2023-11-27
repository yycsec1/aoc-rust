use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, line_ending, u8};
use nom::IResult;
use nom::multi::{ separated_list1};
use nom::sequence::delimited;

use super::{Stacks, Move};
pub fn parse_empty(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, None))
}

pub fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, crate_char) = delimited(tag("["), anychar, tag("]"))(input)?;
    Ok((input, Some(crate_char)))
}

pub fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = u8(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = u8(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = u8(input)?;
    Ok((input, Move{amount, from, to}))
}


pub fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let (input, crates_hrz) = separated_list1(tag(" "), alt((parse_empty, parse_crate)))(input)?;
    Ok((input, crates_hrz))
}

pub fn parse_input(input: &str) -> IResult<&str, (Stacks, Vec<Move>)> {
    let (stack_input, remaining) = input.split_once("1").unwrap();
    let (_, move_input) = remaining.split_once("\r\n\r\n").unwrap();
    let (_, stacks_hrz) = separated_list1(line_ending, parse_crate_line)(stack_input)?;
    let (_, moves) = separated_list1(line_ending, parse_move)(move_input)?;
    Ok(("", (stacks_hrz.into(),moves)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        assert_eq!(parse_empty("   "), Ok(("", None)));
    }

    #[test]
    fn test_parse_crate() {
        assert_eq!(parse_crate("[A]"), Ok(("", Some('A'))));
        assert_eq!(parse_crate("[B]"), Ok(("", Some('B'))));
        assert_eq!(parse_crate("[C]"), Ok(("", Some('C'))));
    }

    #[test]
    fn test_parse_crate_line() {
        assert_eq!(parse_crate_line("    [D]    "), Ok(("", vec![None, Some('D'), None])));
        assert_eq!(parse_crate_line("[N] [C]    "), Ok(("", vec![Some('N'), Some('C'), None])));
        assert_eq!(parse_crate_line("[Z] [M] [P]"), Ok(("", vec![Some('Z'), Some('M'), Some('P')])));
        assert_eq!(parse_crate_line("[Z] [M] [P]\n 1"), Ok(("\n 1", vec![Some('Z'), Some('M'), Some('P')])));
    }
}