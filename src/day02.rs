use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    character::complete::u32,
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

pub struct Game {
    id: u32,
    sets: Vec<Set>,
}

struct Set(Vec<Cubes>);

enum Cubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                delimited(tag("Game "), u32, tag(": ")),
                separated_list1(tag("; "), Set::parse),
            )),
            |(id, sets)| Self { id, sets },
        )(input)
    }

    fn id_if_possible(&self) -> Option<u32> {
        self.sets.iter().all(Set::possible).then_some(self.id)
    }

    fn power(&self) -> u32 {
        let (mut red, mut green, mut blue) = (u32::MIN, u32::MIN, u32::MIN);

        for set in &self.sets {
            for cube in &set.0 {
                match cube {
                    Cubes::Red(n) => red = red.max(*n),
                    Cubes::Green(n) => green = green.max(*n),
                    Cubes::Blue(n) => blue = blue.max(*n),
                };
            }
        }

        red * green * blue
    }
}

impl Set {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(tag(", "), Cubes::parse), Self)(input)
    }

    fn possible(&self) -> bool {
        self.0.iter().all(Cubes::possible)
    }
}

impl Cubes {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(u32, space1, alt((tag("red"), tag("green"), tag("blue")))),
            |(n, color)| match color {
                "red" => Cubes::Red(n),
                "green" => Cubes::Green(n),
                "blue" => Cubes::Blue(n),
                unknown => panic!("unknown color {unknown}"),
            },
        )(input)
    }

    fn possible(&self) -> bool {
        match self {
            Self::Red(n) => *n <= 12,
            Self::Green(n) => *n <= 13,
            Self::Blue(n) => *n <= 14,
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(|l| Game::parse(l).unwrap().1).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {
    input.iter().filter_map(Game::id_if_possible).sum::<u32>()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> u32 {
    input.iter().map(Game::power).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
		 Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
		 Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
		 Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
		 Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            8
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
		 Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
		 Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
		 Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
		 Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            2286
        );
    }
}
