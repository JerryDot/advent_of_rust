use std::collections::HashSet;

use itertools::Itertools;
static INPUT: &str = include_str!("../input/input4.txt");

fn parse_input(input_string: &str) -> Vec<Vec<&str>> {
    input_string
        .trim()
        .split('\n')
        .map(|s| {
            s.split(' ')
             .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>()
}

fn part_one(passphrase_list: Vec<Vec<&str>>) -> i32 {
    let mut total = 0;
    for phrase in passphrase_list {
        let mut dictionary = HashSet::<&str>::new();
        let mut found = false;
        for word in phrase {
            if dictionary.contains(word) {
                found = true;
                break
            }
            dictionary.insert(word);
        }
        if !found {
            total += 1;
        }
    }
    total
}

fn part_two(passphrase_list: Vec<Vec<&str>>) -> i32 {
    let mut total = 0;
    for phrase in passphrase_list {
        let mut dictionary = HashSet::<&str>::new();
        let mut found = false;
        for word in phrase {
            if dictionary.contains(&word.chars().sorted().collect::<String::from>()) {
                found = true;
                break
            }
            dictionary.insert(word.chars().sorted().collect());
        }
        if !found {
            total += 1;
        }
    }
    total
}

advent_of_rust::main! {
    let cleaned_input = parse_input(INPUT);
    let part_one = part_one(cleaned_input);
    println!("{}", part_one);
    // let part_two = part_two(cleaned_input);
    (part_one, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part_one_answer() {
    //     assert_eq!(part_one(&parse_input(INPUT)), 36174)
    // }

    // #[test]
    // fn part_two_answer() {
    //     assert_eq!(part_two(&parse_input(INPUT)) as u16, 244)
    // }
}

