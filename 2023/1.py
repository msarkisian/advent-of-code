import unittest


def part_1(input: str) -> int:
    lines = input.split("\n")
    sum = 0
    for line in lines:
        if not line:
            break
        first_num, last_num = None, None
        for char in line:
            if char.isnumeric():
                if not first_num:
                    first_num, last_num = char, char
                else:
                    last_num = char
        sum += 10 * int(first_num) + int(last_num)
    return sum


class Test(unittest.TestCase):
    def test_part_1(self):
        test_str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"
        self.assertEqual(part_1(test_str), 142)


if __name__ == "__main__":
    with open("./input/1.txt") as file:
        print(f"Part 1: {part_1(file.read())}")
