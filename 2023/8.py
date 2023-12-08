import unittest
import itertools
from collections import namedtuple
import math


def part_1(input: str) -> int:
    lines = input.splitlines()
    directions, nodes = lines[0], lines[2:]

    node_map = {}
    for node in nodes:
        tokens = node.split(' ')
        node_name = tokens[0]
        left_link = tokens[2].strip('(').strip(',')
        right_link = tokens[3].strip(')')
        node_map[node_name] = (left_link, right_link)

    current_node = "AAA"
    step_count = 0
    for step in itertools.cycle(directions):
        if step == 'L':
            current_node = node_map[current_node][0]
        elif step == 'R':
            current_node = node_map[current_node][1]
        else:
            raise Exception(f"unexpected step {step} in directions")
        step_count += 1
        if current_node == "ZZZ":
            return step_count


def part_2(input: str) -> int:
    lines = input.splitlines()
    directions, nodes = lines[0], lines[2:]

    start_nodes = []

    node_map = {}
    for node in nodes:
        tokens = node.split(' ')
        node_name = tokens[0]
        if node_name[2] == 'A':
            start_nodes.append(node_name)
        left_link = tokens[2].strip('(').strip(',')
        right_link = tokens[3].strip(')')
        node_map[node_name] = (left_link, right_link)
    Cycle = namedtuple("Cycle", ["first_len", "period"])
    cycles = []
    for node in start_nodes:
        found_once = False
        current_node = node
        for step_count, step in enumerate(itertools.cycle(directions), start=1):
            if step == 'L':
                current_node = node_map[current_node][0]
            elif step == 'R':
                current_node = node_map[current_node][1]
            else:
                raise Exception(f"unexpected step {step} in directions")
            if current_node[2] == 'Z':
                if not found_once:
                    first_len = step_count
                    found_once = True
                else:
                    cycle_len = step_count - first_len
                    cycles.append(Cycle(first_len, cycle_len))
                    break
    # `first_len` and `period` are the same for all cycles in the input!
    # therefore, for THIS SPECIFIC PROBLEM INPUT ONLY, we can take a simple LCM
    return math.lcm(*[p.period for p in cycles])


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("../input/2023/8_testa.txt") as f:
            self.assertEqual(part_1(f.read()), 6)

    def test_part_2(self):
        with open("../input/2023/8_testb.txt") as f:
            self.assertEqual(part_2(f.read()), 6)


if __name__ == "__main__":
    with open("../input/2023/day8.txt") as f:
        input = f.read()
        print(f"Part 1: {part_1(input)}")
        print(f"Part 2: {part_2(input)}")
