use itertools::Itertools;

static INPUT: &str = include_str!("../input/input2.txt");

fn parse_input(input_string: &str) -> Vec<Vec<u32>> {
    input_string
        .trim()
        .split('\n')
        .map(|s| {
            s.split('\t')
                .map(|w| {
                    w.parse::<u32>()
                        .unwrap_or_else(|w| panic!("Err {}  - wasn't a valid u32", w))
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn part_one(input_array: &[Vec<u32>]) -> u32 {
    input_array
        .iter()
        .map(|l| l.iter().max().unwrap() - l.iter().min().unwrap())
        .sum()
}

fn part_two(input_array: &[Vec<u32>]) -> f64 {
    let x = input_array
        .iter()
        .map(|l| {
            l.iter()
                .combinations(2)
                .filter(|a| a[0] % a[1] == 0 || a[1] % a[0] == 0)
                .sorted()
                .collect::<Vec<Vec<&u32>>>()
        })
        .collect::<Vec<Vec<Vec<&u32>>>>()
        .iter()
        .map(|t| {
            let z = &t[0];
            *z[0].max(z[1]) as f64 / *z[0].min(z[1]) as f64
        })
        .sum();
    x
}

advent_of_rust::main! {
    let cleaned_input = parse_input(INPUT);
    let part_one = part_one(&cleaned_input);
    let part_two = part_two(&cleaned_input);
    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_answer() {
        assert_eq!(part_one(&parse_input(INPUT)), 36174)
    }

    #[test]
    fn part_two_answer() {
        assert_eq!(part_two(&parse_input(INPUT)) as u16, 244)
    }
}
