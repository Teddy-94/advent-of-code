
with open("..\input\day7.txt", "r") as f:
    d = [int(x) for x in f.read().split(",")]
    d.sort()


def part1():
    fuel = 0
    for n in d:
        fuel += abs(n-d[len(d)//2])

    return fuel


def part2():
    bestCost = 2147483647
    for position in range(max(d)):
        cost = 0
        for crabStart in d:
            moves = abs(crabStart-position)
            cost += moves*(moves+1)//2
        if cost < bestCost:
            bestCost = cost
    return bestCost


print(part1(), part2())
