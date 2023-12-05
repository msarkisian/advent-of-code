import unittest
import dataclasses


@dataclasses.dataclass
class Range():
    start: int
    end: int

    @property
    def length(self):
        return self.end - self.start + 1


@dataclasses.dataclass
class RangeMap():
    start: int
    length: int
    offset: int

    @property
    def end(self):
        return self.start + self.length - 1

    def map_val(self, val: int) -> int | None:
        if self.start <= val < self.start + self.length:
            return val + self.offset
        return None


def map_ranges(ranges: list[Range], range_maps: list[RangeMap]) -> list[Range]:
    queue = ranges
    output = []
    # for r in queue:
    while len(queue) > 0:
        r = queue.pop()
        found = False
        for rm in range_maps:
            if r.start >= rm.start and r.end <= rm.end:
                # rm perfectly contains range
                output.append(Range(r.start + rm.offset, r.end + rm.offset))
                found = True
                break
            elif r.start >= rm.start and r.start <= rm.end and r.end > rm.end:
                # range overflows rm end
                output.append(Range(r.start + rm.offset, rm.end + rm.offset))
                queue.append(Range(rm.end + 1, r.end))
                found = True
                break
            elif r.start < rm.start and r.end >= rm.start and r.end <= rm.end:
                # range overflows rm start

                output.append(Range(rm.start + rm.offset, r.end + rm.offset))
                queue.append(Range(r.start, rm.start - 1))
                found = True
                break
            elif r.start < rm.start and r.end > rm.end:
                # range overflows on both ends
                output.append(Range(rm.start + rm.offset, rm.end + rm.offset))
                queue.append(Range(r.start, rm.start - 1))
                queue.append(Range(rm.end + 1, r.end))
                found = True
                break
        # no matches found on any map
        if not found:
            output.append(r)
    return output


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
        ranges.append(RangeMap(int(vals[1]), int(
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


def part_2(input: str) -> int:
    lines = input.splitlines()
    seeds, maps = lines[0].split(' ')[1:], lines[2:]
    seed_idx = 0
    ranges = []
    while seed_idx < len(seeds):
        start = int(seeds[seed_idx])
        length = int(seeds[seed_idx + 1])
        ranges.append(Range(start, start + length - 1))
        seed_idx += 2

    map_idx = 1
    range_maps = []
    while map_idx < len(maps):
        if maps[map_idx] == '':
            # process
            ranges = map_ranges(ranges, range_maps)
            range_maps = []

            map_idx += 2
            continue
        vals = maps[map_idx].split(' ')
        range_maps.append(RangeMap(int(vals[1]), int(
            vals[2]), int(vals[0]) - int(vals[1])))
        map_idx += 1
    return min(ranges, key=lambda r: r.start).start


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

    def test_part_2(self):
        with open("../input/2023/5_test.txt") as f:
            self.assertEqual(part_2(f.read()), 46)

    def test_map_ranges(self):
        # perfect
        ranges = [
            Range(79, 92),
        ]
        range_maps = [
            RangeMap(79, 14, -1)
        ]
        self.assertEqual(map_ranges(ranges, range_maps), [Range(78, 91)])


if __name__ == "__main__":
    with open("../input/2023/day5.txt") as f:
        input = f.read()
        print(f"Part 1: {part_1(input)}")
        print(f"Part 2: {part_2(input)}")
