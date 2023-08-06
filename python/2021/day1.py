with open("..\input\day1", "r") as f:
    d = f.read().strip().splitlines()


def part1():
    depth_increases = 0
    for i in range(1, len(d)):
        if (d[i] > d[i-1]):
            depth_increases += 1
    return depth_increases


def part2():
    depth_increases = 0
    for i in range(2, len(d)):
        if (d[i-3]+d[i-2]+d[i-1] < d[i-2]+d[i-1]+d[i]):
            depth_increases += 1
    return depth_increases


print(part1(), part2())
