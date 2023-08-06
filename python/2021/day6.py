with open("..\input\day6.txt", "r") as f:
    fishes = [int(x) for x in f.read().split(",")]
    buckets = [0] * 9

    for fish in fishes:
        buckets[fish] += 1


def part1(buckets):
    for _ in range(80):
        zero = buckets[0]
        buckets = buckets[1:] + [zero]
        buckets[6] += zero
    return sum(buckets)


def part2(buckets):
    for _ in range(256):
        zero = buckets[0]
        buckets = buckets[1:] + [zero]
        buckets[6] += zero
    return sum(buckets)


print(part1(buckets), part2(buckets))
