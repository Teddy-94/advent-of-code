with open("..\input\day5.txt", "r") as f:
    d = f.read().strip().splitlines()

    points = [line.split("->") for line in d]
    startPoints = [x[0].split(",") for x in points]
    endPoints = [x[1].split(",") for x in points]

    startX = [int(x[0].strip()) for x in startPoints]
    startY = [int(y[1].strip()) for y in startPoints]
    endX = [int(x[0].strip()) for x in endPoints]
    endY = [int(y[1].strip()) for y in endPoints]


def part1():
    grid = {}
    count = 0
    for i in range(len(points)):

        if startX[i] == endX[i]:
            for y in range(min(startY[i], endY[i]), max(startY[i], endY[i])+1):
                if (startX[i], y) in grid:
                    grid[(startX[i], y)] += 1
                else:
                    grid[(startX[i], y)] = 1

        if startY[i] == endY[i]:
            for x in range(min(startX[i], endX[i]), max(startX[i], endX[i])+1):
                if (x, startY[i]) in grid:
                    grid[(x, startY[i])] += 1
                else:
                    grid[(x, startY[i])] = 1
    for i in grid:
        if grid[i] > 1:
            count += 1

    return count


def part2():
    grid = {}
    count = 0
    for i in range(len(points)):
        if startX[i] < endX[i]:
            xDirection = 1
        elif startX[i] > endX[i]:
            xDirection = -1
        else:
            xDirection = 0

        if startY[i] < endY[i]:
            yDirection = 1
        elif startY[i] > endY[i]:
            yDirection = -1
        else:
            yDirection = 0

        x = startX[i]
        y = startY[i]
        while (x, y) != (endX[i] + xDirection, endY[i] + yDirection):
            if (x, y) in grid:
                grid[(x, y)] += 1
            else:
                grid[(x, y)] = 1
            x = x + xDirection
            y = y + yDirection

    for i in grid:
        if grid[i] > 1:
            count += 1

    return count


print(part1(), part2())
