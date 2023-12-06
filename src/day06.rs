pub struct Input {
    times: Vec<u64>,
    distances: Vec<u64>,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    let distances = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    Input { times, distances }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    input
        .times
        .iter()
        .zip(input.distances.iter())
        .map(|(t, d)| {
            (0..=*t)
                .map(|hold| hold * (t - hold))
                .filter(|traveled| traveled > d)
                .count() as u64
        })
        .product()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    let time = input
        .times
        .iter()
        .map(|n| n.to_string())
        .reduce(|mut acc, el| {
            acc.push_str(&el);
            acc
        })
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let distance = input
        .distances
        .iter()
        .map(|n| n.to_string())
        .reduce(|mut acc, el| {
            acc.push_str(&el);
            acc
        })
        .unwrap()
        .parse::<u64>()
        .unwrap();

    solve_part1(&Input {
        times: vec![time],
        distances: vec![distance],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30\n\
			   Distance:  9  40  200";

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 288);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 71503);
    }
}
