import re
import unittest
import math


def is_possible(reds: int, greens: int, blues: int) -> bool:
    MAX_REDS = 12
    MAX_GREENS = 13
    MAX_BLUES = 14
    return (reds <= MAX_REDS and greens <= MAX_GREENS and blues <= MAX_BLUES)


def part_1(input: str) -> int:
    total = 0
    for line in input.splitlines():
        rounds = re.split(r'[:;]', line)
        game_id, rounds = int(rounds[0].split(" ")[1].strip(":")), rounds[1::]

        game_disqualified = False
        for round in rounds:
            if game_disqualified:
                break
            color_counts = {"red": 0, "green": 0, "blue": 0}
            tokens = round.strip().split(" ")
            curr_token = 0
            while curr_token < len(tokens):
                color_counts[tokens[curr_token +
                                    1].strip(',')] += int(tokens[curr_token])
                curr_token += 2
            if not is_possible(color_counts["red"], color_counts["green"], color_counts["blue"]):
                game_disqualified = True
        if not game_disqualified:
            total += game_id

    return total


def part_2(input: str) -> int:
    total = 0
    for line in input.splitlines():
        rounds = re.split(r'[:;]', line)[1::]
        max_color_counts = {"red": 0, "green": 0, "blue": 0}
        for round in rounds:
            tokens = round.strip().split(" ")
            curr_token = 0
            while curr_token < len(tokens):
                color = tokens[curr_token + 1].strip(',')
                max_color_counts[color] = max(
                    max_color_counts[color], int(tokens[curr_token]))
                curr_token += 2
        total += math.prod(max_color_counts.values())
    return total


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("./input/2_test.txt") as file:

            self.assertEqual(part_1(file.read()), 8)

    def test_part_2(self):
        with open("./input/2_test.txt") as file:
            self.assertEqual(part_2(file.read()), 2286)


if __name__ == "__main__":
    with open("./input/2.txt") as file:
        input = file.read()
        print(f"Part 1: {part_1(input)}")
        print(f"Part 2: {part_2(input)}")
