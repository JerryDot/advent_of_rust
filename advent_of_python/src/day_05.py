from copy import deepcopy
import os

def parse_input():
    script_dir = os.path.dirname(__file__)
    rel_path = "../../input/input5.txt"
    with open(os.path.join(script_dir, rel_path), 'r') as input_file:
        input_string = list(map(lambda k: int(k), input_file.readlines()))
        other_input = deepcopy(input_string)
    return input_string, other_input

def part_one(input_list) -> int:
    pointer = 0
    counter = 0
    instructions_length = len(input_list)
    while 0 <= pointer < instructions_length:
        input_list[pointer] += 1
        pointer = pointer + input_list[pointer] - 1
        counter += 1
    return counter

def part_two(input_list) -> int:
    pointer = 0
    counter = 0
    instructions_length = len(input_list)
    while 0 <= pointer < instructions_length:
        if input_list[pointer] > 2:
            input_list[pointer] -= 1
            pointer = pointer + input_list[pointer] + 1
        else:
            input_list[pointer] += 1
            pointer = pointer + input_list[pointer] - 1
        counter += 1
    return counter


if __name__ == '__main__': 
    input_1, input_2 = parse_input()
    part_uno = part_one(input_1)
    part_dos = part_two(input_2)
    print(part_uno)
    print(part_dos)

