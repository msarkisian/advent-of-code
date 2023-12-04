import unittest
import re


def part_1(input: str) -> int:
    total = 0
    for line in input.splitlines():
        [winning_numbers, my_numbers] = line.split('|')
        winning_numbers = set(re.split(
            r'\s+', winning_numbers)[2:-1])
        my_numbers = re.split(r'\s+', my_numbers)[1:]

        # print(winning_numbers)
        score = 0
        for num in my_numbers:
            if num in winning_numbers:
                if score != 0:
                    score *= 2
                else:
                    score = 1
        # print(score)
        total += score
    return total


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("../input/2023/4_test.txt") as file:
            self.assertEqual(part_1(file.read()), 13)


if __name__ == "__main__":
    with open("../input/2023/day4.txt") as file:
        input = file.read()
        print(f"Part 1: {part_1(input)}")
        # print(f"Part 2: {part_2(input)}")
