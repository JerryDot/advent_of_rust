use itertools::multizip;

static INPUT: &str = include_str!("../input/input1.txt");

fn parse_input(input_string: &str) -> Vec<u32> {
    input_string
        .trim()
        .chars()
        .map(|c| c as u32 - '0' as u32)
        .collect::<Vec<u32>>()
}

fn part_one(input: &Vec<u32>) -> u32 {
    let mut last_digit = 0;
    let mut sum = (input.last().unwrap() == &input[0]) as u32 * &input[0];
    for number in input {
        if last_digit == *number {
            sum += number;
        }
        last_digit = *number;
    }
    sum
}

fn part_two(input: &Vec<u32>) -> u32 {
    let length: usize = input.len();
    let first_half = &input[0..length / 2];
    let second_half = &input[(length / 2)..length];
    multizip((first_half, second_half))
        .filter(|(a,b)| a == b)
        .map(|(a, _b)| 2 * a)
        .sum()
}

advent_of_rust::main! {
    let cleaned_input = parse_input(INPUT);
    let part_one = part_one(&cleaned_input);
    let part_two = part_two(&cleaned_input);
    (part_one, part_two)
}
