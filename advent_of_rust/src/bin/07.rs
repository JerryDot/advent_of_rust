
use regex::Regex;
use lazy_static::lazy_static;
use itertools::structs::Unique;


static INPUT: String = std::string::String::from(include_str!("../../../input/input7.txt"));

// fn parse_input(input_string: &str) ->  {
//     input_string
//         .trim()
//         .
//         .collect::<Vec<Vec<&str>>>()
// }

fn part_one(input: String) -> &'static str {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[a-z]+").unwrap();
    }
    let llllisto: Vec<String> = vec![];
    for word in RE.captures_iter(&input) {
        llllisto.append(word as String);
    }
    let listo = RE.captures_iter(&input).filter_map(|w| w).iter().collect::<Vec<str>>();
    let x = "hi";

    x

}

// fn part_two() ->  {

// }

advent_of_rust::main! {
    // let cleaned_input = parse_input(INPUT);
    let part_one = part_one(INPUT);
    // let part_two = part_two(cleaned_input);
    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_answer() {
        assert_eq!(part_one(&parse_input(INPUT)), 1)
    }

    #[test]
    fn part_two_answer() {
        assert_eq!(part_two(parse_input(INPUT)), 2)
    }
}
