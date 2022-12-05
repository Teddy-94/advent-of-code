with open("..\..\input\day9.txt") as f:
    d = f.readlines()

    grid = []
    for line in d:
        grid.append([int(number) for number in line.strip()])

    row_len = len(grid)
    column_len = len(grid[0])

    delta_row = [0, 0, -1, 1]
    delta_column = [1, -1, 0, 0]


def part1():
    total_score = 0
    for row in range(row_len):
        for column in range(column_len):

            lowpoint = True
            # loop through each neighbour
            for d in range(4):
                row_check = row + delta_row[d]
                column_check = column + delta_column[d]

                # bounds checking
                if not ((0 <= row_check and row_check < row_len) and (0 <= column_check and column_check < column_len)):
                    continue

                # Neighbour depth checking
                if grid[row_check][column_check] <= grid[row][column]:
                    lowpoint = False
                    break

            if lowpoint:
                total_score += grid[row][column] + 1

    return total_score


def part2():
    basins = []
    visited = set()

    # Depth first search
    for row in range(row_len):
        for column in range(column_len):

            if (row, column) not in visited and grid[row][column] != 9:
                basin_size = 0
                stack = []
                stack.append((row, column))

                # go through each item in the stack
                while len(stack) > 0:
                    # pick it off the stack when we check it
                    (row, column) = stack.pop()

                    # check if we have visited this point in the grid
                    if (row, column) in visited:
                        continue
                    
                    # mark that we have visited a point
                    visited.add((row, column))
                    basin_size += 1

                    # check each neighbour of the point if they are in bounds and !=9
                    for d in range(4):
                        row_check = row + delta_row[d]
                        column_check = column + delta_column[d]

                        if 0 <= row_check < row_len and 0 <= column_check < column_len and grid[row_check][column_check] != 9:
                            # if the point passes this check, it needs to be visited
                            stack.append((row_check, column_check))
                # here we are finished with the basin, and add the count to our basins
                basins.append(basin_size)

    basins.sort()
    return basins, (basins[-1] * basins[-2] * basins[-3])


print(part2())
