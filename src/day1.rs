use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|s| s.parse::<u16>().unwrap())
        .collect::<Vec<u16>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<u16>) -> usize {
    calc_part1(input)
}

pub fn calc_part1(input: &Vec<u16>) -> usize {
    input.iter().zip(input.iter().skip(1)).fold(
        0,
        |acc, (prev, cur)| {
            if cur > prev {
                acc + 1
            } else {
                acc
            }
        },
    )
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<u16>) -> usize {
    // Transform input into an average of 3
    // and then call calc_part1
    let new_input = input
        .iter()
        .zip(input.iter().skip(1).zip(input.iter().skip(2)))
        .map(|(d1, (d2, d3))| d1 + d2 + d3)
        .collect::<Vec<u16>>();
    calc_part1(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT),
            vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 7);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 5);
    }
}
