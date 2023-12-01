use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

fn one(input: &str) -> IResult<&str, u32> {
    map(alt((tag("one"), tag("1"))), |_| 1)(input)
}

fn two(input: &str) -> IResult<&str, u32> {
    map(alt((tag("two"), tag("2"))), |_| 2)(input)
}

fn three(input: &str) -> IResult<&str, u32> {
    map(alt((tag("three"), tag("3"))), |_| 3)(input)
}

fn four(input: &str) -> IResult<&str, u32> {
    map(alt((tag("four"), tag("4"))), |_| 4)(input)
}

fn five(input: &str) -> IResult<&str, u32> {
    map(alt((tag("five"), tag("5"))), |_| 5)(input)
}

fn six(input: &str) -> IResult<&str, u32> {
    map(alt((tag("six"), tag("6"))), |_| 6)(input)
}

fn seven(input: &str) -> IResult<&str, u32> {
    map(alt((tag("seven"), tag("7"))), |_| 7)(input)
}

fn eight(input: &str) -> IResult<&str, u32> {
    map(alt((tag("eight"), tag("8"))), |_| 8)(input)
}

fn nine(input: &str) -> IResult<&str, u32> {
    map(alt((tag("nine"), tag("9"))), |_| 9)(input)
}

fn parse_digit(input: &str) -> IResult<&str, u32> {
    alt((one, two, three, four, five, six, seven, eight, nine))(input)
}

pub struct Calibration(String);

impl Calibration {
    fn as_digits(&self) -> u32 {
        let (mut first, mut last) = (None, None);

        for c in self.0.chars() {
            if let Some(i) = c.to_digit(10) {
                if first.is_none() {
                    first = Some(i);
                }
                last = Some(i);
            }
        }

        first.unwrap() * 10 + last.unwrap()
    }

    fn as_letters(&self) -> u32 {
        let mut cur = 0;

        let first = loop {
            if let Ok((_, i)) = parse_digit(&self.0[cur..]) {
                break i;
            } else {
                cur += 1;
            }
        };

        cur = self.0.len() - 1;

        let last = loop {
            if let Ok((_, i)) = parse_digit(&self.0[cur..]) {
                break i;
            } else {
                cur -= 1;
            }
        };

        first * 10 + last
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Calibration> {
    input
        .lines()
        .map(|line| Calibration(line.to_string()))
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Calibration]) -> u32 {
    input.iter().map(Calibration::as_digits).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Calibration]) -> u32 {
    input.iter().map(Calibration::as_letters).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "1abc2\n\
		 pqr3stu8vwx\n\
		 a1b2c3d4e5f\n\
		 treb7uchet"
            )),
            142
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "two1nine\n\
		 eightwothree\n\
		 abcone2threexyz\n\
		 xtwone3four\n\
		 4nineeightseven2\n\
		 zoneight234\n\
		 7pqrstsixteen"
            )),
            281
        );
    }
}
