import unittest
import re


def part_1(input: str) -> int:
    total = 0
    for line in input.splitlines():
        [winning_numbers, my_numbers] = line.split('|')
        winning_numbers = set(re.split(
            r'\s+', winning_numbers)[2:-1])
        my_numbers = re.split(r'\s+', my_numbers)[1:]

        score = 0
        for num in my_numbers:
            if num in winning_numbers:
                if score != 0:
                    score *= 2
                else:
                    score = 1
        total += score
    return total


def part_2(input: str) -> int:
    cards = input.splitlines()
    num_cards = [1 for _ in range(len(cards))]
    for (card_idx, card) in enumerate(cards):
        [winning_numbers, my_numbers] = card.split('|')
        winning_numbers = set(re.split(
            r'\s+', winning_numbers)[2:-1])
        my_numbers = re.split(r'\s+', my_numbers)[1:]
        wins = 0
        for num in my_numbers:
            if num in winning_numbers:
                wins += 1
        for i in range(card_idx + 1, card_idx + wins + 1):
            num_cards[i] += num_cards[card_idx]
    return sum(num_cards)


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("../input/2023/4_test.txt") as file:
            self.assertEqual(part_1(file.read()), 13)

    def test_part_2(self):
        with open("../input/2023/4_test.txt") as file:
            self.assertEqual(part_2(file.read()), 30)


if __name__ == "__main__":
    with open("../input/2023/day4.txt") as file:
        input = file.read()
        print(f"Part 1: {part_1(input)}")
        print(f"Part 2: {part_2(input)}")
