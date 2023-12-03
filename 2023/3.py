import unittest
import dataclasses


@dataclasses.dataclass
class GridNumber():
    processed: bool
    val: int


def part_1(input: str) -> int:
    symbol_locations = set()
    number_locations: dict[(int, int), GridNumber] = {}
    num_buffer = ""
    loc_buffer = []
    for y, line in enumerate(input.splitlines()):
        for x, char in enumerate(line):
            if char in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]:
                num_buffer += char
                loc_buffer.append((x, y))
            elif num_buffer:
                grid_number = GridNumber(False, int(num_buffer))
                for loc in loc_buffer:
                    number_locations[loc] = grid_number
                num_buffer = ""
                loc_buffer = []
            if not char in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"] and char != '.':
                symbol_locations.add((x, y))
        if num_buffer:
            grid_number = GridNumber(False, int(num_buffer))
            for loc in loc_buffer:
                number_locations[loc] = grid_number
            num_buffer = ""
            loc_buffer = []
    sum = 0
    for (x, y) in symbol_locations:
        # up left
        if (x - 1, y - 1) in number_locations:
            num = number_locations[(x-1, y-1)]
            if not num.processed:
                sum += num.val
                num.processed = True
        # up
        if (x, y - 1) in number_locations:
            num = number_locations[(x, y - 1)]
            if not num.processed:
                sum += num.val
                num.processed = True
        # up right
        if (x + 1, y - 1) in number_locations:
            num = number_locations[(x + 1, y - 1)]
            if not num.processed:
                sum += num.val
                num.processed = True
        # left
        if (x - 1, y) in number_locations:
            num = number_locations[(x - 1, y)]
            if not num.processed:
                sum += num.val
                num.processed = True
        # right
        if (x + 1, y) in number_locations:
            num = number_locations[(x + 1, y)]
            if not num.processed:
                sum += num.val
                num.processed = True
        # down left
        if (x - 1, y + 1) in number_locations:
            num = number_locations[(x - 1, y + 1)]
            if not num.processed:
                sum += num.val
                num.processed = True
        # down
        if (x, y + 1) in number_locations:
            num = number_locations[(x, y + 1)]
            if not num.processed:
                sum += num.val
                num.processed = True
        # down right
        if (x + 1, y + 1) in number_locations:
            num = number_locations[(x + 1, y + 1)]
            if not num.processed:
                sum += num.val
                num.processed = True
    return sum


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("./input/3_test.txt") as file:
            self.assertEqual(part_1(file.read()), 4361)


if __name__ == "__main__":
    with open("./input/3.txt") as file:
        input = file.read()
        print(f"Part 1: {part_1(input)}")
        # print(f"Part 2: {part_2(input)}")
