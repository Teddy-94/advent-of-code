with open("..\input\day2", "r") as f:
    d = f.read().split("\n")
    commands = [line.split(" ") for line in d]


def part1():
    position = depth = 0
    for command in commands:
        if command[0] == "up":
            depth -= int(command[1])
        elif command[0] == "down":
            depth += int(command[1])
        elif command[0] == "forward":
            position += int(command[1])

    return position*depth


def part2():
    position = depth = aim = 0
    for command in commands:
        if command[0] == "up":
            aim -= int(command[1])
        elif command[0] == "down":
            aim += int(command[1])
        elif command[0] == "forward":
            position += int(command[1])
            depth += int(command[1])*aim
    return position*depth


print(part1(), part2())
