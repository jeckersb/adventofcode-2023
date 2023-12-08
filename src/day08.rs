use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, space0, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, pair, separated_pair, tuple},
    IResult,
};

pub struct Documents {
    instructions: Vec<Instruction>,
    network: Network,
}

struct DocumentIter<'a> {
    documents: &'a Documents,
    iter: Box<dyn Iterator<Item = Instruction> + 'a>,
    next: &'a String,
}

#[derive(Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

struct Network(HashMap<String, (String, String)>);

impl Documents {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                many1(Instruction::parse),
                pair(newline, newline),
                Network::parse,
            ),
            |(instructions, network)| Self {
                instructions,
                network,
            },
        )(input)
    }

    fn iter(&self, start: &str) -> DocumentIter {
        DocumentIter {
            documents: self,
            iter: Box::new(self.instructions.iter().copied().cycle()),
            next: self.network.0.get_key_value(start).unwrap().0,
        }
    }

    fn step(&self, node: &String, instruction: Instruction) -> Option<&String> {
        self.network.step(node, instruction)
    }
}

impl<'a> Iterator for DocumentIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.next;

        self.next = self.documents.step(res, self.iter.next().unwrap()).unwrap();

        Some(res)
    }
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("L"), |_| Self::Left),
            map(tag("R"), |_| Self::Right),
        ))(input)
    }
}

impl Network {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, mappings) = separated_list1(
            newline,
            separated_pair(
                Network::parse_nodename,
                tuple((space0, tag("="), space0)),
                Network::parse_destination,
            ),
        )(input)?;

        Ok((
            rest,
            Self(
                mappings
                    .into_iter()
                    .map(|(src, (left, right))| {
                        (src.to_string(), (left.to_string(), right.to_string()))
                    })
                    .collect(),
            ),
        ))
    }

    fn parse_nodename(input: &str) -> IResult<&str, &str> {
        alphanumeric1(input)
    }

    fn parse_destination(input: &str) -> IResult<&str, (&str, &str)> {
        delimited(
            tag("("),
            separated_pair(
                Network::parse_nodename,
                pair(tag(","), space1),
                Network::parse_nodename,
            ),
            tag(")"),
        )(input)
    }

    fn step(&self, node: &String, instruction: Instruction) -> Option<&String> {
        let (left, right) = self.0.get(node)?;

        match instruction {
            Instruction::Left => Some(left),
            Instruction::Right => Some(right),
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Documents {
    Documents::parse(input).unwrap().1
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Documents) -> usize {
    input.iter("AAA").position(|node| node == "ZZZ").unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Documents) -> usize {
    let a_keys = input.network.0.keys().filter(|key| key.ends_with('A'));

    let mut iters: Vec<_> = a_keys.map(|k| input.iter(k)).collect();

    let results: Vec<_> = iters
        .iter_mut()
        .map(|i| i.position(|s| s.ends_with('Z')).unwrap())
        .collect();

    results
        .into_iter()
        .reduce(|acc, i| num::integer::lcm(i, acc))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "RL\n\
			     \n\
			     AAA = (BBB, CCC)\n\
			     BBB = (DDD, EEE)\n\
			     CCC = (ZZZ, GGG)\n\
			     DDD = (DDD, DDD)\n\
			     EEE = (EEE, EEE)\n\
			     GGG = (GGG, GGG)\n\
			     ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_2: &str = "LLR\n\
			     \n\
			     AAA = (BBB, BBB)\n\
			     BBB = (AAA, ZZZ)\n\
			     ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE_1)), 2);
        assert_eq!(solve_part1(&input_generator(EXAMPLE_2)), 6);
    }

    const EXAMPLE_3: &str = "LR\n\
			     \n\
			     11A = (11B, XXX)\n\
			     11B = (XXX, 11Z)\n\
			     11Z = (11B, XXX)\n\
			     22A = (22B, XXX)\n\
			     22B = (22C, 22C)\n\
			     22C = (22Z, 22Z)\n\
			     22Z = (22B, 22B)\n\
			     XXX = (XXX, XXX)";

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE_3)), 6);
    }
}
