use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Schematic(Vec<Vec<SchematicCell>>);

#[derive(Debug, Default)]
struct Number {
    value: u32,
    adjacent: bool,
    visited: bool,
}

#[derive(Debug)]
enum SchematicCell {
    Empty,
    NumberPlaceholder,
    Number(Rc<RefCell<Number>>),
    Symbol(char),
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl Number {
    fn visit_if_adjacent(&mut self) -> Option<u32> {
        if !self.adjacent {
            return None;
        }

        if self.visited {
            return None;
        }

        self.visited = true;
        Some(self.value)
    }

    fn mark_adjacent(&mut self) {
        self.adjacent = true;
    }
}

impl Schematic {
    fn mark_adjacencies(&mut self) {
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let SchematicCell::Number(ref n) = cell {
                    if self
                        .iter_adj(x, y)
                        .any(|cell| matches!(cell, SchematicCell::Symbol(_)))
                    {
                        n.borrow_mut().mark_adjacent();
                    }
                }
            }
        }
    }

    fn iter_part_nums(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().flatten().filter_map(|cell| match cell {
            SchematicCell::Number(n) => n.borrow_mut().visit_if_adjacent(),
            _ => None,
        })
    }

    fn iter_adj(&self, x: usize, y: usize) -> impl Iterator<Item = &SchematicCell> {
        const OFFSETS: [(isize, isize); 8] = [
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
        ];

        OFFSETS.iter().filter_map(move |(xoff, yoff)| {
            self.0
                .get(((y as isize) + yoff) as usize)
                .and_then(|row| row.get(((x as isize) + xoff) as usize))
        })
    }

    fn iter_symbols(&self, symbol: char) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.0.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().filter_map(move |(x, cell)| {
                matches!(cell, SchematicCell::Symbol(s) if *s == symbol ).then_some((x, y))
            })
        })
    }

    fn iter_gears(&self) -> impl Iterator<Item = u32> + '_ {
        self.iter_symbols('*')
            .filter_map(|(x, y)| self.gear_value_at(x, y))
    }

    fn gear_value_at(&self, x: usize, y: usize) -> Option<u32> {
        let mut nums = Vec::new();

        for cell in self.iter_adj(x, y) {
            if let SchematicCell::Number(n) = cell {
                if nums
                    .iter()
                    .any(|other: &Rc<RefCell<Number>>| Rc::ptr_eq(n, other))
                {
                    continue;
                }

                nums.push(Rc::clone(n));
            }
        }

        if nums.len() == 2 {
            Some(nums[0].borrow().value * nums[1].borrow().value)
        } else {
            None
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Schematic {
    let mut cells = Vec::new();
    let mut digits = String::new();

    for line in input.lines() {
        let mut row_cells = Vec::new();

        for (i, ch) in line.chars().enumerate() {
            match ch {
                '0'..='9' => {
                    digits.push(ch);
                    row_cells.push(SchematicCell::NumberPlaceholder);
                }
                '.' => {
                    if !digits.is_empty() {
                        let len = digits.len();
                        let n = Rc::new(RefCell::new(Number::from(digits.parse::<u32>().unwrap())));
                        digits.clear();

                        for cell in &mut row_cells[i - len..i] {
                            *cell = SchematicCell::Number(Rc::clone(&n));
                        }
                    }
                    row_cells.push(SchematicCell::Empty);
                }
                s => {
                    if !digits.is_empty() {
                        let len = digits.len();
                        let n = Rc::new(RefCell::new(Number::from(digits.parse::<u32>().unwrap())));
                        digits.clear();

                        for cell in &mut row_cells[i - len..i] {
                            *cell = SchematicCell::Number(Rc::clone(&n));
                        }
                    }

                    row_cells.push(SchematicCell::Symbol(s));
                }
            }
        }

        if !digits.is_empty() {
            let len = digits.len();
            let n = Rc::new(RefCell::new(Number::from(digits.parse::<u32>().unwrap())));
            digits.clear();

            let start = row_cells.len() - len;
            let end = row_cells.len();
            for cell in &mut row_cells[start..end] {
                *cell = SchematicCell::Number(Rc::clone(&n));
            }
        }

        assert!(row_cells
            .iter()
            .all(|cell| !matches!(cell, SchematicCell::NumberPlaceholder)));
        cells.push(row_cells);
    }

    let mut s = Schematic(cells);
    s.mark_adjacencies();
    s
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Schematic) -> u32 {
    input.iter_part_nums().sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Schematic) -> u32 {
    input.iter_gears().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..\n\
			   ...*......\n\
			   ..35..633.\n\
			   ......#...\n\
			   617*......\n\
			   .....+.58.\n\
			   ..592.....\n\
			   ......755.\n\
			   ...$.*....\n\
			   .664.598..";

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 4361);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 467835);
    }
}
