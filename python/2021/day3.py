with open("..\input\day3") as f:
    lines = f.read().strip().splitlines()


def part1():
    count = [0] * len(lines[0])

    for line in lines:
        for i, char in enumerate(line):
            if char == "1":
                count[i] += 1
            else:
                count[i] -= 1

    gamma = ""
    for i in count:
        if i <= 0:
            gamma += "0"
        else:
            gamma += "1"
    epsilon = "".join(chr(ord(char) ^ 1) for char in gamma)

    return(int(gamma, 2)*int(epsilon, 2))


def part2():
    oxygen = lines
    carbondioxide = lines
    i = 0
    while len(oxygen) > 1 or len(carbondioxide) > 1:

        if len(oxygen) > 1:
            oxygenZeroes = len([x for x in oxygen if x[i] == "0"])
            oxygenOnes = len([x for x in oxygen if x[i] == "1"])
            if oxygenOnes >= oxygenZeroes:
                oxygen = [x for x in oxygen if x[i] == "1"]
            else:
                oxygen = [x for x in oxygen if x[i] == "0"]

        if len(carbondioxide) > 1:
            carbondioxideZeroes = len(
                [x for x in carbondioxide if x[i] == "0"])
            carbondixideOnes = len([x for x in carbondioxide if x[i] == "1"])
            if carbondixideOnes >= carbondioxideZeroes:
                carbondioxide = [x for x in carbondioxide if x[i] == "0"]
            else:
                carbondioxide = [x for x in carbondioxide if x[i] == "1"]
        i += 1

    return(int(oxygen[0], 2) * int(carbondioxide[0], 2))


print(part1(), part2())
