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


def part_2(input: str) -> int:
    digits = {
        "1": 1,
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }
    sum = 0
    for line in input.split("\n"):
        if not line:
            break
        leftmost_idx, leftmost_digit = None, None
        rightmost_idx, rightmost_digit = None, None
        for digit in digits.keys():
            idx = line.find(digit)
            if idx != -1 and ((leftmost_idx == None) or idx < leftmost_idx):
                leftmost_idx, leftmost_digit = idx, digit
            idx = line.rfind(digit)
            if idx != -1 and ((rightmost_idx == None) or idx > rightmost_idx):
                rightmost_idx, rightmost_digit = idx, digit
        sum += 10 * digits[leftmost_digit] + digits[rightmost_digit]
    return sum


class Test(unittest.TestCase):
    def test_part_1(self):
        test_str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"
        self.assertEqual(part_1(test_str), 142)

    def test_part_2(self):
        test_str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"
        self.assertEqual(part_2(test_str), 281)

    def test_part_2_first_line(self):
        test_str = "two1nine"
        self.assertEqual(part_2(test_str), 29)


if __name__ == "__main__":
    with open("./input/1.txt") as file:
        input = file.read()
        print(f"Part 1: {part_1(input)}")
        print(f"Part 2: {part_2(input)}")
