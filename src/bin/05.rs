static INPUT: &str = include_str!("../input/input5.txt");

fn parse_input(input_string: &str) -> Vec<i32> {
    input_string
        .trim()
        .split('\n')
        .map(|w|
            w.parse::<i32>()
             .unwrap_or_else(|w| panic!("Err {}  - wasn't a valid u32", w)))
        .collect::<Vec<i32>>()
}

fn part_one(instructions: Vec<i32>) -> u16 {
    let pointer: i32 = 0;
    let counter = 0;
    let instructions_length = instructions.len() as i16;
    while pointer >= 0 && pointer < instructions_length {
        instructions[pointer] += 1;
        pointer += instructions[pointer] - 1;
        counter += 1
    }
    counter
}

fn part_two() -> i32 {
    4
}

advent_of_rust::main! {
    let cleaned_input = parse_input(INPUT);
    let part_one = part_one(cleaned_input);
    let part_two = part_two(cleaned_input);
    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_answer() {
        assert_eq!(part_one(&parse_input(INPUT)), <ANSWER_ONE>)
    }

    #[test]
    fn part_two_answer() {
        assert_eq!(part_two(parse_input(INPUT)), <ANSWER_TWO>)
    }
}
