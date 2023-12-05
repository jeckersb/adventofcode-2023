use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::u32,
    character::complete::{newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

pub struct Almanac {
    seeds: Seeds,
    maps: Maps,
}

struct Seeds(Vec<u32>);

struct Maps(Vec<Map>);

struct Map(Vec<MapItem>);

struct MapItem {
    src: Range<u32>,
    dst: Range<u32>,
}

impl Almanac {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(Seeds::parse, pair(newline, newline), Maps::parse),
            |(seeds, maps)| Self { seeds, maps },
        )(input)
    }

    fn iter_locations(&self) -> impl Iterator<Item = u32> + '_ {
        self.seeds.iter().map(|seed| self.maps.location(seed))
    }

    fn iter_locations2(&self) -> impl Iterator<Item = u32> + '_ {
        self.seeds.chunks(2).flat_map(|range| {
            (range[0]..(range[0] + range[1])).map(|seed| self.maps.location(seed))
        })
    }
}

impl Seeds {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(preceded(tag("seeds: "), separated_list1(space1, u32)), Self)(input)
    }

    fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().copied()
    }

    fn chunks(&self, chunk_size: usize) -> impl Iterator<Item = &[u32]> {
        self.0.chunks(chunk_size)
    }
}

impl Maps {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(pair(newline, newline), Map::parse), Self)(input)
    }

    fn location(&self, seed: u32) -> u32 {
        self.0.iter().fold(seed, |acc, map| map.location(acc))
    }
}

impl Map {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(Map::parse_heading, separated_list1(newline, MapItem::parse)),
            Self,
        )(input)
    }

    fn parse_heading(input: &str) -> IResult<&str, &str> {
        terminated(take_until(" "), tuple((space1, tag("map:"), newline)))(input)
    }

    fn location(&self, seed: u32) -> u32 {
        for item in &self.0 {
            if let Some(loc) = item.location(seed) {
                return loc;
            }
        }

        seed
    }
}

impl MapItem {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((u32, space1, u32, space1, u32)),
            |(dst, _, src, _, len)| Self {
                src: src..src + len,
                dst: dst..dst + len,
            },
        )(input)
    }

    fn location(&self, seed: u32) -> Option<u32> {
        if self.src.contains(&seed) {
            let offset = seed - self.src.start;
            Some(self.dst.start + offset)
        } else {
            None
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Almanac {
    Almanac::parse(input).unwrap().1
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Almanac) -> u32 {
    input.iter_locations().min().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Almanac) -> u32 {
    input.iter_locations2().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13\n\
			   \n\
			   seed-to-soil map:\n\
			   50 98 2\n\
			   52 50 48\n\
			   \n\
			   soil-to-fertilizer map:\n\
			   0 15 37\n\
			   37 52 2\n\
			   39 0 15\n\
			   \n\
			   fertilizer-to-water map:\n\
			   49 53 8\n\
			   0 11 42\n\
			   42 0 7\n\
			   57 7 4\n\
			   \n\
			   water-to-light map:\n\
			   88 18 7\n\
			   18 25 70\n\
			   \n\
			   light-to-temperature map:\n\
			   45 77 23\n\
			   81 45 19\n\
			   68 64 13\n\
			   \n\
			   temperature-to-humidity map:\n\
			   0 69 1\n\
			   1 0 69\n\
			   \n\
			   humidity-to-location map:\n\
			   60 56 37\n\
			   56 93 4";

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 35);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 46);
    }
}
