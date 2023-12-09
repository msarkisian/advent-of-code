import unittest


def part_1(input: str) -> int:
    def calculate_differences(input: list[int]) -> list[int]:
        differences = []
        for i in range(len(input) - 1):
            differences.append(input[i + 1] - input[i])
        return differences

    def find_next(input: list[int]) -> int:
        if all(map(lambda x: x == 0, input)):
            return 0
        return input[-1] + find_next(calculate_differences(input))

    total = 0
    for line in input.splitlines():
        total += find_next([*map(lambda x: int(x), line.split(' '))])
    return total


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("../input/2023/9_test.txt") as f:
            self.assertEqual(part_1(f.read()), 114)

    # def test_part_2(self):
    #     with open("../input/2023/9_test.txt") as f:
    #         self.assertEqual(part_2(f.read()), 6)


if __name__ == "__main__":
    with open("../input/2023/day9.txt") as f:
        input = f.read()
        print(f"Part 1: {part_1(input)}")
        # print(f"Part 2: {part_2(input)}")
