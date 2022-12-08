with open("..\input\day5.txt", "r") as f:
    d = f.read().strip().splitlines()

    points = [line.split("->") for line in d]
    startPoints = [x[0].split(",") for x in points]
    endPoints = [x[1].split(",") for x in points]

    startX = [int(x[0].strip()) for x in startPoints]
    startY = [int(y[1].strip()) for y in startPoints]
    endX = [int(x[0].strip()) for x in endPoints]
    endY = [int(y[1].strip()) for y in endPoints]

    #print(f"startpoints {startPoints}, endpoints {endPoints}")

    grid = {}


def part1():
    count = 0
    for i in range(len(points)):
        # look for horizontal or vertical lines only
        if startX[i] == endX[i]:
            #print(min(startY[i], endY[i]))
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
    print(grid)
    return count
def part2():
    count = 0
    
    return count

    # if startY[i] == endY[i]:
    #print(startX[i], startY[i], endX[i], endY[i])

    #print([x[0] for x in startPoints])
    # startPoints[i][0] endPoints[i][0] -> X
    # startPoints[i][1] endPoints[i][1] -> Y
    # print(f"start {startPoints[i]}, end {endPoints[i]}")
    # if i == len(points)-1:
    #     print(f"startX {startPoints[i][0]}, endS {endPoints[i][0]}")

    # for line in d:
    #     startPoint, endPoint = line.split("->")

#         x1, y1 = tuple(map(int, p1.split(",")))
#         x2, y2 = tuple(map(int, p2.split(",")))

#         dx = x2-x1
#         dy = y2-y1

#         for i in range(max(abs(dx), abs(dy)) + 1):
#             x = x1 + (1 if dx > 0 else (-1 if dx < 0 else 0))*i
#             y = y1 + (1 if dy > 0 else (-1 if dy < 0 else 0))*i

#             if dx == 0 or dy == 0:
#                 points[(x, y)] += 1
#             points2[(x, y)] += 1

# count = 0
# for n in points:
#     if points[n] > 1:
#         count += 1
# print(count)

# count2 = 0
# for n in points2:
#     if points2[n] > 1:
#         count2 += 1
# print(count2)


def markGrid(startX, startY, endX, endY):
    dx = endX - startX
    dy = endY - startY


print(part1())
