from enum import Enum


class Direction(Enum):
    NORTH = 1
    EAST = 2
    SOUTH = 3
    WEST = 4

    def left(self):
        match self:
            case Direction.NORTH:
                return Direction.WEST
            case Direction.WEST:
                return Direction.SOUTH
            case Direction.SOUTH:
                return Direction.EAST
            case Direction.EAST:
                return Direction.NORTH

    def right(self):
        match self:
            case Direction.NORTH:
                return Direction.EAST
            case Direction.WEST:
                return Direction.NORTH
            case Direction.SOUTH:
                return Direction.WEST
            case Direction.EAST:
                return Direction.SOUTH


def part1(directions: list[str]):
    facing = Direction.NORTH
    x, y = 0, 0
    for dir in directions:
        match dir[0]:
            case 'R':
                facing = facing.right()
            case 'L':
                facing = facing.left()
            case other:
                raise Exception("invalid direction in input")
        steps = int(dir[1:])
        match facing:
            case Direction.NORTH:
                y += steps
            case Direction.SOUTH:
                y -= steps
            case Direction.EAST:
                x += steps
            case Direction.WEST:
                x -= steps
    return abs(x) + abs(y)


# TEST
# directions = ["R2", "L3"]
# print(part1(directions))  # 5

# directions = ["R2", "R2", "R2"]
# print(part1(directions))  # 2

# directions = ["R5", "L5", "R5", "R3"]
# print(part1(directions))  # 12

def part2(directions: list[str]):
    facing = Direction.NORTH
    x, y = 0, 0
    visited_locations = set()

    for dir in directions:
        match dir[0]:
            case 'R':
                facing = facing.right()
            case 'L':
                facing = facing.left()
            case other:
                raise Exception("invalid direction in input")
        steps = int(dir[1:])
        for _ in range(steps):
            match facing:
                case Direction.NORTH:
                    y += 1
                case Direction.SOUTH:
                    y -= 1
                case Direction.EAST:
                    x += 1
                case Direction.WEST:
                    x -= 1
            if str(x) + "," + str(y) in visited_locations:
                return abs(x) + abs(y)
            visited_locations.add(str(x) + ',' + str(y))
    print(visited_locations)


# TEST
# directions = ["R8", "R4", "R4", "R8"]
# print(part2(directions))  # 4


if __name__ == "__main__":
    with open("../input/2016/day1.txt") as file:
        directions = file.read().split(", ")
        print(part1(directions))
        print(part2(directions))
