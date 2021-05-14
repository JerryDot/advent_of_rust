// #![feature(test)]

// extern crate test;

pub static INPUT: &str = include_str!("../src/input/input5.txt");

pub fn parse_input(input_string: &str) -> Vec<i32> {
    input_string
        .trim()
        .split('\n')
        .map(|w|
            w.parse::<i32>()
             .unwrap_or_else(|w| panic!("Err {}  - wasn't a valid u32", w)))
        .collect::<Vec<i32>>()
}

fn part_one(mut instructions: Vec<i32>) -> u32 {
    let mut pointer: usize = 0;
    let mut counter = 0;
    let instructions_length = instructions.len() as u32;
    while pointer < instructions_length as usize {
        instructions[pointer] += 1;
        pointer = pointer + instructions[pointer] as usize - 1;
        counter += 1;
    }
    counter
}

pub fn part_two(mut instructions: Vec<i32>) -> u32 {
    let mut pointer: usize = 0;
    let mut counter = 0;
    let instructions_length = instructions.len() as u32;
    while pointer < instructions_length as usize {
        if instructions[pointer] > 2 {
            instructions[pointer] -= 1;
            pointer = pointer + instructions[pointer] as usize + 1;
        } else {
            instructions[pointer] += 1;
            pointer = pointer + instructions[pointer] as usize - 1;
        }
        counter += 1;
    }
    counter
}

advent_of_rust::main! {
    let cleaned_input = parse_input(INPUT);
    let part_one = part_one(cleaned_input);
    let recleaned_input = parse_input(INPUT);    
    let part_two = part_two(recleaned_input);
    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    #[test]
    fn part_one_answer() {
        assert_eq!(part_one(parse_input(INPUT)), 372139)
    }

    // #[bench]
    // fn bench_add_two(b: &mut Bencher) {
    //     let p1_input = parse_input(INPUT);
    //     b.iter(|| part_one(p1_input))
    // }


    #[test]
    fn part_two_answer() {
        assert_eq!(part_two(parse_input(INPUT)), 29629538)
    }
}