import statistics
with open ("..\..\input\day7.txt", "r") as f:
    d = f.read().split(",")
    for i, line in enumerate(d):
        d[i] = int(d[i])
    d.sort()

def part1():
    median = int(statistics.median(d))
    fuel = 0
    for n in d:
        fuel += abs(n-median)

    return fuel 

def part2():
    bestCost = 2147483647
    for position in range(max(d)):
        cost = 0
        for crabStart in d:
            moves = abs(crabStart-position)
            cost += moves*(moves+1)/2
        if cost < bestCost:
            bestCost = cost
    return int(bestCost)

print(part1(), part2())