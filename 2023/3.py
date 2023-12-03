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


def part_2(input: str) -> int:
    gear_locations = set()
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
            if char == '*':
                gear_locations.add((x, y))
        if num_buffer:
            grid_number = GridNumber(False, int(num_buffer))
            for loc in loc_buffer:
                number_locations[loc] = grid_number
            num_buffer = ""
            loc_buffer = []
    total = 0
    for (x, y) in gear_locations:
        found_nums: list[GridNumber] = []
        # up left
        if (x - 1, y - 1) in number_locations:
            num = number_locations[(x-1, y-1)]
            if not num.processed:
                found_nums.append(num)
                num.processed = True
        # up
        if (x, y - 1) in number_locations:
            num = number_locations[(x, y - 1)]
            if not num.processed:
                found_nums.append(num)
                num.processed = True
        # up right
        if (x + 1, y - 1) in number_locations:
            num = number_locations[(x + 1, y - 1)]
            if not num.processed:
                found_nums.append(num)
                num.processed = True
        # left
        if (x - 1, y) in number_locations:
            num = number_locations[(x - 1, y)]
            if not num.processed:
                found_nums.append(num)
                num.processed = True
        # right
        if (x + 1, y) in number_locations:
            num = number_locations[(x + 1, y)]
            if not num.processed:
                found_nums.append(num)
                num.processed = True
        # down left
        if (x - 1, y + 1) in number_locations:
            num = number_locations[(x - 1, y + 1)]
            if not num.processed:
                found_nums.append(num)
                num.processed = True
        # down
        if (x, y + 1) in number_locations:
            num = number_locations[(x, y + 1)]
            if not num.processed:
                found_nums.append(num)
                num.processed = True
        # down right
        if (x + 1, y + 1) in number_locations:
            num = number_locations[(x + 1, y + 1)]
            if not num.processed:
                found_nums.append(num)
                num.processed = True
        if len(found_nums) == 2:
            total += found_nums[0].val * found_nums[1].val
        for fn in found_nums:
            fn.processed = False
    return total


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("../input/2023/3_test.txt") as file:
            self.assertEqual(part_1(file.read()), 4361)

    def test_part_2(self):
        with open("../input/2023/3_test.txt") as file:
            self.assertEqual(part_2(file.read()), 467835)


if __name__ == "__main__":
    with open("../input/2023/day3.txt") as file:
        input = file.read()
        print(f"Part 1: {part_1(input)}")
        print(f"Part 2: {part_2(input)}")
