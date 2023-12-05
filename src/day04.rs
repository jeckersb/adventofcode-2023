use nom::{
    bytes::complete::tag,
    character::complete::u32,
    character::complete::{space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Clone)]
pub struct Game {
    id: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                delimited(pair(tag("Card"), space1), u32, pair(tag(":"), space1)),
                preceded(space0, separated_list1(space1, u32)),
                tag(" | "),
                preceded(space0, separated_list1(space1, u32)),
            )),
            |(id, winning, _pipe, have)| Self { id, winning, have },
        )(input)
    }

    fn matches(&self) -> usize {
        self.have
            .iter()
            .filter(|have| self.winning.iter().any(|win| win == *have))
            .count()
    }

    fn score(&self) -> u32 {
        let matches = self.matches();

        if matches == 0 {
            return 0;
        }

        2u32.pow((matches as u32) - 1)
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(|g| Game::parse(g).unwrap().1).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {
    input.iter().map(Game::score).sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Game]) -> u32 {
    let mut scores = vec![0; input.len() + 1];

    for game in input.iter().rev() {
        let mut score = 1;
        let matches = game.matches();
        let mut begin = game.id as usize + 1;
        let mut end = game.id as usize + 1 + matches;

        if begin > scores.len() {
            begin = scores.len();
        }

        if end > scores.len() {
            end = scores.len();
        }

        score += &scores[begin..end].iter().sum();

        scores[game.id as usize] = score;
    }

    scores.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
			   Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
			   Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
			   Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
			   Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
			   Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 13);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 30);
    }
}
