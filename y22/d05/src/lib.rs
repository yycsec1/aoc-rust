mod parser;
use parser::*;

#[derive(Debug)]
pub struct Move {
    amount: u8,
    from: u8,
    to: u8,
}

#[derive(Debug)]
pub struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn crate_mover_9000_move(&mut self, m: &Move) {
        for _ in 0..m.amount {
            let c = self.stacks[m.from as usize - 1].pop().unwrap();
            self.stacks[m.to as usize - 1].push(c);
        }
    }

    fn crate_mover_9001_move(&mut self, m: &Move) {
        let mut moved_crates = Vec::new();
        for _ in 0..m.amount {
            let c = self.stacks[m.from as usize - 1].pop().unwrap();
            moved_crates.push(c);
        }
        for c in moved_crates.iter().rev() {
            self.stacks[m.to as usize - 1].push(*c);
        }
    }

    fn get_top_crates(&self) -> String {
        self.stacks.iter().map(|stack| stack.iter().last().unwrap()).collect()
    }
}

impl From<Vec<Vec<Option<char>>>> for Stacks {
    fn from(value: Vec<Vec<Option<char>>>) -> Self {
        let Some(max_len) = value.iter().map(|inner| inner.len()).max() else {return Stacks{stacks: Vec::new()}};
        let mut stacks = Vec::with_capacity(max_len);
        for _ in 0..max_len {
            stacks.push(Vec::with_capacity(value.len()));
        }
        for row in value.iter().rev() {
            for (i, ele) in row.iter().enumerate() {
                if let Some(c) = ele {
                    stacks[i].push(*c);
                }
            }
        }
        Stacks{stacks}
    }
}

pub fn part_a(input: &str) -> String {
    let (_, (mut stacks, moves)) = parse_input(input).unwrap();
    for m in &moves {
        stacks.crate_mover_9000_move(m);
    }
    stacks.get_top_crates()
}

pub fn part_b(input: &str) -> String {
    let (_, (mut stacks, moves)) = parse_input(input).unwrap();
    for m in &moves {
        stacks.crate_mover_9001_move(m);
    }
    stacks.get_top_crates()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_sample() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_a(input), "CMZ".to_string());
    }

    #[test]
    fn test_part_a() {
        let input = include_str!("../input.txt");
        assert_eq!(part_a(input), "QNHWJVJZW".to_string());
    }

    #[test]
    fn test_part_b_sample() {
        let input = include_str!("../sample.txt");
        assert_eq!(part_b(input), "MCD".to_string());
    }

    #[test]
    fn test_part_b() {
        let input = include_str!("../input.txt");
        assert_eq!(part_b(input), "BPCZJLFJW".to_string());
    }
}
