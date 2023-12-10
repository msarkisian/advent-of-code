import unittest
from enum import Enum
import sys


class Direction(Enum):
    Up = 0
    Right = 1
    Down = 2
    Left = 3


def part_1(input: str) -> int:
    def search(x: int, y: int, dir: Direction, steps: int):
        match pipe_map[y][x]:
            case 'S':
                return steps // 2
            case '|':
                if dir == Direction.Up:
                    return search(x, y+1, Direction.Up, steps + 1)
                if dir == Direction.Down:
                    return search(x, y-1, Direction.Down, steps + 1)
            case '-':
                if dir == Direction.Left:
                    return search(x + 1, y, Direction.Left, steps+1)
                if dir == Direction.Right:
                    return search(x - 1, y, Direction.Right, steps+1)
            case 'L':
                if dir == Direction.Up:
                    return search(x + 1, y, Direction.Left, steps+1)
                if dir == Direction.Right:
                    return search(x, y - 1, Direction.Down, steps+1)
            case 'J':
                if dir == Direction.Left:
                    return search(x, y - 1, Direction.Down, steps+1)
                if dir == Direction.Up:
                    return search(x - 1, y, Direction.Right, steps+1)
            case '7':
                if dir == Direction.Left:
                    return search(x, y + 1, Direction.Up, steps+1)
                if dir == Direction.Down:
                    return search(x - 1, y, Direction.Right, steps+1)
            case 'F':
                if dir == Direction.Right:
                    return search(x, y + 1, Direction.Up, steps+1)
                if dir == Direction.Down:
                    return search(x + 1, y, Direction.Left, steps+1)
            case c:
                raise Exception(f"pipe led to unexpected tile {c}")

    pipe_map = []
    for y, line in enumerate(input.splitlines()):
        pipe_map.append([])
        for x, char in enumerate(line):
            pipe_map[y].append(char)
            if char == 'S':
                start_point = (x, y)
    start_x, start_y = start_point

    if start_x > 0 and pipe_map[start_y][start_x - 1] in ['-', 'L', 'F']:
        return search(start_x - 1, start_y, Direction.Right, 1)
    if start_x < len(pipe_map[0]) - 1 and pipe_map[start_y][start_x + 1] in ['-', 'J', '7']:
        return search(start_x + 1, start_y, Direction.Left, 1)
    if start_y > 0 and pipe_map[start_y - 1][start_x] in ['|', '7', 'F']:
        return search(start_x, start_y - 1, Direction.Up, 1)
    if start_y < len(pipe_map) - 1 and pipe_map[start_y + 1][start_x] in ['|', 'L', 'J']:
        return search(start_x, start_y + 1, Direction.Down, 1)
    raise Exception("cannot find location to begin search")


class Test(unittest.TestCase):
    def test_part_1(self):
        with open("../input/2023/10_test.txt") as f:
            self.assertEqual(part_1(f.read()), 8)

    # def test_part_2(self):
    #     with open("../input/2023/10_test.txt") as f:
    #         self.assertEqual(part_2(f.read()), 2)


if __name__ == "__main__":
    sys.setrecursionlimit(100000)
    with open("../input/2023/day10.txt") as f:
        input = f.read()
        print(f"Part 1: {part_1(input)}")
        # print(f"Part 2: {part_2(input)}")
