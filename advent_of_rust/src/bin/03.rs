use std::{
    collections::{hash_map::RandomState, HashMap},
    thread::current,
};
static INPUT: &str = include_str!("../../../input/input3.txt");

fn parse_input(input_string: &str) -> u32 {
    input_string
        .trim()
        .parse::<u32>()
        .unwrap_or_else(|a| panic!("Err '{}' wasn't a valid u32: {}", input_string, a))
}

fn part_one(square: u32) -> u32 {
    let x = (square as f64).sqrt() as u32 + 1;
    println!("{}", x);
    371
}

const EAST: [i8; 2] = [1, 0];
const NORTH: [i8; 2] = [0, 1];
const WEST: [i8; 2] = [-1, 0];
const SOUTH: [i8; 2] = [0, -1];

enum direction {
    EAST,
    NORTH,
    WEST,
    SOUTH,
}

// fn change_direction(direction: [])
// pub struct HashMap<(i8, i8), u16, S = RandomState>
type Coord = [i8; 2];

fn new_coord_from<F: Iterator<Item = i8>>(src: F) -> Coord {
    let mut result: [i8; 2] = [0; 2];
    for (rref, val) in result.iter_mut().zip(src) {
        *rref = val;
    }
    result
}

fn add(a: Coord, b: Coord) -> Coord {
    new_coord_from(a.iter().zip(&b).map(|(a, b)| a + b))
}

fn part_two(limit: u32) -> u16 {
    panic!("Not implemented");
/*
    let mut current_size = 1;
    let mut position: [i8; 2] = [0, 0];
    let mut change: [i8; 2] = EAST;
    let mut size_store: HashMap<[i8; 2], u32, RandomState> = HashMap::new();
    let mut counter = 0;
    size_store.insert(position, current_size);

    let most_EAST = 0;
    let most_NORTH = 0;
    let most_WEST = 0;
    let most_SOUTH = 0;
    const adjacent_moves: [[i8; 2]; 8] = [
        [0, 1],
        [1, 0],
        [1, 1],
        [-1, 0],
        [0, -1],
        [-1, -1],
        [-1, 1],
        [1, -1],
    ];

    while current_size < limit {
        position = add(position, change);
        if position[0] > most_EAST {
            change = NORTH;
        } else if position[0] < most_NORTH {
            change = WEST
        } else if position[1] > most_WEST {
            change = SOUTH
        } else if position[1] < most_SOUTH {
            change = EAST
        };
        current_size = adjacent_moves
            .iter()
            .map(|a| {
                if size_store.contains_key(add(new_coord_from(&position), *a)) {
                    size_store[add(&position, *a)]
                } else {
                    0
                }
            })
            .sum()
    }
*/
    10
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
        assert_eq!(part_one(parse_input(INPUT)), 371)
    }

    // #[test]
    // fn part_two_answer() {
    //     assert_eq!(part_two(parse_input(INPUT)) as u16, 244)
    // }
}
