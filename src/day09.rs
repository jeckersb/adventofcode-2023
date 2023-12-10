#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect()
}

fn next(values: &[i32]) -> i32 {
    let mut ends = vec![*values.last().unwrap()];
    let mut values = values.to_owned();

    loop {
        values = values.windows(2).map(|pairs| pairs[1] - pairs[0]).collect();

        if values.iter().all(|n| *n == 0) {
            break;
        }

        ends.push(*values.last().unwrap());
    }

    ends.iter().sum()
}

fn prev(values: &[i32]) -> i32 {
    let mut heads = vec![*values.first().unwrap()];
    let mut values = values.to_owned();

    loop {
        values = values.windows(2).map(|pairs| pairs[1] - pairs[0]).collect();

        if values.iter().all(|n| *n == 0) {
            break;
        }

        heads.push(*values.first().unwrap());
    }

    heads
        .iter()
        .rev()
        .copied()
        .reduce(|acc, i| i - acc)
        .unwrap()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|i| next(i)).sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|i| prev(i)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15\n\
			   1 3 6 10 15 21\n\
			   10 13 16 21 30 45";
    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 114);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(prev(&[10, 13, 16, 21, 30, 45]), 5);
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 2);
    }
}
