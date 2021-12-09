use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<String>) -> usize {
    let coords = input.iter().fold((0, 0), |mut acc: (i32, i32), line| {
        let mut blah = line.split(" ");
        let direction = blah.next().unwrap();
        let coord = blah.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => acc.0 += coord,
            "down" => acc.1 += coord,
            "up" => acc.1 -= coord,
            _ => panic!("Invalid direction"),
        }

        acc
    });
    (coords.0 * coords.1) as usize
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<String>) -> usize {
    let coords = input
        .iter()
        .fold((0, 0, 0), |mut acc: (i32, i32, i32), line| {
            let mut blah = line.split(" ");
            let direction = blah.next().unwrap();
            let coord = blah.next().unwrap().parse::<i32>().unwrap();

            match direction {
                "forward" => {
                    acc.0 += coord;
                    acc.1 += coord * acc.2
                }
                "down" => acc.2 += coord,
                "up" => acc.2 -= coord,
                _ => panic!("Invalid direction"),
            }

            acc
        });
    (coords.0 * coords.1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(INPUT),
            vec![String::from("1"), String::from("2"), String::from("3")]
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 150);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 900);
    }
}
