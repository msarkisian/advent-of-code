import unittest
import itertools


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


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("../input/2023/8_test.txt") as f:
            self.assertEqual(part_1(f.read()), 6)

    # def test_part_2(self):
    #     with open("../input/2023/8_test.txt") as f:
    #         self.assertEqual(part_2(f.read()), 5905)


if __name__ == "__main__":
    with open("../input/2023/day8.txt") as f:
        input = f.read()
        print(f"Part 1: {part_1(input)}")
        # print(f"Part 2: {part_2(input)}")
