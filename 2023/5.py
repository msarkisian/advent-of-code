import unittest
import dataclasses


@dataclasses.dataclass
class Range():
    start: int
    length: int
    offset: int

    def map_val(self, val: int) -> int | None:
        if self.start <= val < self.start + self.length:
            return val + self.offset
        return None


def part_1(input: str) -> int:
    lines = input.splitlines()
    seeds, maps = map(lambda x: int(x), lines[0].split(' ')[1:]), lines[2:]
    map_idx = 1
    ranges = []
    while map_idx < len(maps):
        if maps[map_idx] == '':
            new_seeds = []
            for seed in seeds:
                found = False
                for range in ranges:
                    if range.map_val(seed) is not None:
                        new_seeds.append(range.map_val(seed))
                        found = True
                        break
                if not found:
                    new_seeds.append(seed)
            ranges = []
            seeds = new_seeds
            map_idx += 2
            continue
        vals = maps[map_idx].split(' ')
        ranges.append(Range(int(vals[1]), int(
            vals[2]), int(vals[0]) - int(vals[1])))
        map_idx += 1

    locations = []
    for seed in seeds:
        found = False
        for range in ranges:
            if range.map_val(seed) is not None:
                locations.append(range.map_val(seed))
                found = True
                break
        if not found:
            locations.append(seed)
    return min(locations)


class Test(unittest.TestCase):
  # Expected transformations:
  # [79, 14, 55, 13]
  # [81, 14, 57, 13]
  # [81, 53, 57, 52]
  # [81, 49, 53, 41]
  # [74, 42, 46, 34]
  # [78, 42, 82, 34]
  # [78, 43, 82, 35]
  # [82, 43, 86, 35]
    def test_part_1(self):
        with open("../input/2023/5_test.txt") as f:
            self.assertEqual(part_1(f.read()), 35)


if __name__ == "__main__":
    with open("../input/2023/day5.txt") as f:
        input = f.read()
        print(f"Part 1: {part_1(input)}")
        # print(f"Part 2: {part_2(input)}")
