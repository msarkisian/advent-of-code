import unittest
import re
import math


def part_1(input: str) -> int:
    lines = input.splitlines()
    times = re.split(r"\s+", lines[0])[1:]
    distances = re.split(r"\s+", lines[1])[1:]

    ways = []

    for time, distance in zip(times, distances):
        time, distance = int(time), int(distance)
        best_time, mod = divmod(time, 2)
        ways_found = 0
        if (best_time * (time - best_time)) > distance:
            if mod != 0:
                ways_found += 2
            else:
                ways_found += 1

        this_time = best_time - 1
        while (this_time * (time - this_time)) > distance:
            ways_found += 2
            this_time -= 1
        ways.append(ways_found)
    return math.prod(ways)


def part_2(input: str) -> int:
    lines = input.splitlines()
    time = int(re.sub(r'\s+', '', lines[0].strip("Time: ")))
    distance = int(re.sub(r'\s+', '', lines[1].strip("Distance: ")))

    best_time, mod = divmod(time, 2)
    ways_found = 0
    if (best_time * (time - best_time)) > distance:
        if mod != 0:
            ways_found += 2
        else:
            ways_found += 1
    this_time = best_time - 1
    while (this_time * (time - this_time)) > distance:
        ways_found += 2
        this_time -= 1
    return ways_found


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("../input/2023/6_test.txt") as f:
            self.assertEqual(part_1(f.read()), 288)

    def test_part_2(self):
        with open("../input/2023/6_test.txt") as f:
            self.assertEqual(part_2(f.read()), 71503)


if __name__ == "__main__":
    with open("../input/2023/day6.txt") as f:
        input = f.read()
        print(f"Part 1: {part_1(input)}")
        print(f"Part 2: {part_2(input)}")
