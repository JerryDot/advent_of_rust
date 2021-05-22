use std::collections::HashSet;

static INPUT: &str = include_str!("../../../input/input6.txt");

fn parse_input(input_string: &str) -> Vec<u16> {
    input_string
        .trim()
        .split('\t')
        .map(|w|
            w.parse::<u16>()
             .unwrap_or_else(|w| panic!("Err {}  - wasn't a valid u16", w)))
        .collect::<Vec<u16>>()
}

fn one_step_replace(banks: Vec<u16>) -> Vec<u16> {
    let length = banks.len() as u16;
    let biggest = *banks.iter().max().unwrap();
    let indexx = banks.iter().position(|&r| r == biggest).unwrap() as u16;
    banks.iter().enumerate().map(|(_i, w)| {
        let mut adder = 0;
        if ((_i as u16 - indexx) % length + length - 1) % length < biggest % length {
            adder = 1;
        };
        if _i as u16 == indexx {
            (biggest / length) + adder
        } else {
            w + (biggest / length) + adder
        }
    }).collect::<Vec<u16>>()
}

fn part_one(banks: Vec<u16>) -> u16 {
    let mut dictionary = HashSet::<Vec<u16>>::new();
    dictionary.insert(banks.clone());
    let mut new_banks = one_step_replace(banks);
    let mut counter = 1;
    while !dictionary.contains(&new_banks) {
        dictionary.insert(new_banks.clone());
        new_banks = one_step_replace(new_banks);
        counter += 1
    }
    counter
}

fn part_two(banks: Vec<u16>) -> u16 {
    let mut dictionary = HashSet::<Vec<u16>>::new();
    dictionary.insert(banks.clone());
    let mut new_banks = one_step_replace(banks);
    while !dictionary.contains(&new_banks) {
        dictionary.insert(new_banks.clone());
        new_banks = one_step_replace(new_banks);
    }
    part_one(new_banks)
}

advent_of_rust::main! {
    let cleaned_input = parse_input(INPUT);
    let part_one = part_one(cleaned_input.clone());
    let part_two = part_two(cleaned_input);
    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_answer() {
        println!("{:?}", one_step_replace(vec![3, 1, 2, 3]));
        println!("{:?}", one_step_replace(vec![5, 1, 2, 3]));
        println!("{:?}", one_step_replace(vec![3, 1, 4, 3]));
        println!("{:?}", one_step_replace(vec![3, 2, 2, 2]));

        assert_eq!(one_step_replace(vec![3, 1, 2, 3]), [0, 2, 3, 4]);
        assert_eq!(one_step_replace(vec![0, 2, 7, 0]), [2, 4, 1, 2]);
        assert_eq!(one_step_replace(vec![2, 4, 1, 2]), [3, 1, 2, 3]);

        assert_eq!(part_one(vec![4, 2]), 2);

        assert_eq!(part_one(parse_input(INPUT)), 6681);
    }

    #[test]
    fn part_two_answer() {
        assert_eq!(part_two(parse_input(INPUT)), 2392)
    }
}

// 4024 << part_one << 1078105