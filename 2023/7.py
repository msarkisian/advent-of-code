import unittest
from enum import Enum
from functools import cmp_to_key


class HandType(Enum):
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,


class Hand:
    cards: list[int]
    hand_type: HandType

    def __init__(self, card_string: str):
        self.cards = []
        card_counts = {}
        for card in card_string:
            match card:
                case "A":
                    val = 14
                case "K":
                    val = 13
                case "Q":
                    val = 12
                case "J":
                    val = 11
                case "T":
                    val = 10
                case c:
                    val = int(c)
            self.cards.append(val)
            if val in card_counts:
                card_counts[val] += 1
            else:
                card_counts[val] = 1

        if len(card_counts) == 5:
            self.hand_type = HandType.HighCard
            return
        if len(card_counts) == 4:
            self.hand_type = HandType.OnePair
            return
        if len(card_counts) == 1:
            self.hand_type = HandType.FiveOfAKind
            return
        max_count = max(card_counts.values())
        if max_count == 4:
            self.hand_type = HandType.FourOfAKind
            return
        if max_count == 3 and len(card_counts) == 2:
            self.hand_type = HandType.FullHouse
            return
        if max_count == 3:
            self.hand_type = HandType.ThreeOfAKind
            return
        if max_count == 2 and len(card_counts) == 3:
            self.hand_type = HandType.TwoPair
            return
        raise Exception(f"cannot calculate type of hand {card_string}")


def compare_hands(hand1: (Hand, int), hand2: (Hand, int)) -> int:
    hand1, hand2 = hand1[0], hand2[0]
    if hand1.hand_type != hand2.hand_type:
        return hand1.hand_type.value[0] - hand2.hand_type.value[0]
    for card1, card2 in zip(hand1.cards, hand2.cards):
        if card1 != card2:
            return card1 - card2
    return 0


def part_1(input: str) -> int:
    hands = []
    for line in input.splitlines():
        [hand_str, bet_str] = line.split(' ')
        hands.append((Hand(hand_str), int(bet_str)))
    hands.sort(key=cmp_to_key(compare_hands))
    total = 0
    for (rank, (_, bet)) in enumerate(hands, start=1):
        total += bet * rank

    return total


class Part2Hand(Hand):
    cards: list[int]
    hand_type: HandType

    def __init__(self, card_string: str):
        self.cards = []
        card_counts = {}
        joker_count = 0
        for card in card_string:
            match card:
                case "A":
                    val = 14
                case "K":
                    val = 13
                case "Q":
                    val = 12
                case "J":
                    val = 1
                    joker_count += 1
                case "T":
                    val = 10
                case c:
                    val = int(c)
            self.cards.append(val)
            if val in card_counts and val != 1:
                card_counts[val] += 1
            elif val != 1:
                card_counts[val] = 1
        max_count = max(card_counts.values(), default=0)
        if max_count + joker_count == 5:
            self.hand_type = HandType.FiveOfAKind
            return
        if max_count + joker_count == 4:
            self.hand_type = HandType.FourOfAKind
            return
        if len(card_counts) == 2:
            self.hand_type = HandType.FullHouse
            return
        if max_count + joker_count == 3:
            self.hand_type = HandType.ThreeOfAKind
            return
        if len(card_counts) == 3:
            self.hand_type = HandType.TwoPair
            return
        if max_count + joker_count == 2:
            self.hand_type = HandType.OnePair
            return
        if max_count + joker_count == 1:
            self.hand_type = HandType.HighCard
            return
        raise Exception(f"cannot calculate type of hand {card_string}")


def part_2(input: str) -> int:
    hands = []
    for line in input.splitlines():
        [hand_str, bet_str] = line.split(' ')
        hands.append((Part2Hand(hand_str), int(bet_str)))
    hands.sort(key=cmp_to_key(compare_hands))
    total = 0
    for (rank, (hand, bet)) in enumerate(hands, start=1):
        total += bet * rank
    return total


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("../input/2023/7_test.txt") as f:
            self.assertEqual(part_1(f.read()), 6440)

    def test_part_2(self):
        with open("../input/2023/7_test.txt") as f:
            self.assertEqual(part_2(f.read()), 5905)


if __name__ == "__main__":
    with open("../input/2023/day7.txt") as f:
        input = f.read()
        print(f"Part 1: {part_1(input)}")
        print(f"Part 2: {part_2(input)}")
