import re

def part1(input_data, iter, width, height):
    robots = parse(input_data)
    hw = width // 2
    hh = height // 2

    for _ in range(iter):
        robots = [move(robot, width, height) for robot in robots]

    robots = [robot[0] for robot in robots]
    robots = [pos for pos in robots if pos[0] != hw and pos[1] != hh]

    # Group by quadrant and calculate product of lengths
    quadrant_map = {}
    for x, y in robots:
        quadrant = (x // (hw + 1), y // (hh + 1))
        quadrant_map[quadrant] = quadrant_map.get(quadrant, 0) + 1

    result = 1
    for count in quadrant_map.values():
        result *= count

    return result

def part2(input_data, iter, width, height):
    robots = parse(input_data)
    pattern = "*" * 31  # Pattern for the Christmas tree

    for i in range(iter):
        robots = [move(robot, width, height) for robot in robots]
        printed = print_grid(robots, width, height)

        if pattern not in printed:
            continue
        else:
            return i

    return None  # If no pattern found

def move(robot, width, height):
    position, velocity = robot
    new_position = add(position, velocity)
    new_position = ((new_position[0] % width), (new_position[1] % height))
    return new_position, velocity

def add(pos1, pos2):
    return (pos1[0] + pos2[0], pos1[1] + pos2[1])

def print_grid(robots, width, height):
    grid = [[' ' for _ in range(width)] for _ in range(height)]

    for x, y in robots:
        grid[y][x] = '*'

    return ''.join(''.join(row) for row in grid)

def parse(input_data):
    robots = []
    for line in input_data:
        match = re.match(r"p=<(-?\d+),(-?\d+)> v=<(-?\d+),(-?\d+)>", line)
        if match:
            position = (int(match[1]), int(match[2]))
            velocity = (int(match[3]), int(match[4]))
            robots.append((position, velocity))
    return robots

# Reading the input file
def read_file(file_path):
    with open(file_path, 'r') as file:
        return file.readlines()

# Example usage
input_data = read_file("14.txt")
iter_count = 100
width = 101
height = 103

result_part1 = part1(input_data, iter_count, width, height)
print(f"Part 1 result: {result_part1}")

result_part2 = part2(input_data, iter_count, width, height)
print(f"Part 2 result: {result_part2}")
