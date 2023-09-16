from collections import deque
INFINITY = float("inf")


class Node:
    def __init__(
            self,
            elevation: str,
            coords: tuple[int, int],
            neighbours: list):
        self.elevation = elevation
        self.coords = coords
        self.neighbours = neighbours

    def setCoords(self, row, col):
        self.coords = (row, col)


def findNeighbours(node: Node, grid):
    def checkElevationDiff(current: Node, end: Node):
        return ord(end.elevation) - ord(current.elevation)

    row = node.coords[0]
    col = node.coords[1]
    npos = row-1
    spos = row+1
    epos = col+1
    wpos = col-1

    neighbours: list[Node] = []
    if npos >= 0:
        N = grid[npos][col]
        if (checkElevationDiff(node, N) <= 1):
            neighbours.append(N)

    if spos < len(grid):
        S = grid[spos][col]
        if (checkElevationDiff(node, S) <= 1):
            neighbours.append(S)

    if epos < len(grid[0]):
        E = grid[row][epos]
        if (checkElevationDiff(node, E) <= 1):
            neighbours.append(E)

    if wpos >= 0:
        W = grid[row][wpos]
        if (checkElevationDiff(node, W) <= 1):
            neighbours.append(W)

    return neighbours


def heuristic(node: Node, end: Node):
    # The heuristic function is a guess  of the cost to reach the end node
    # from the node passed to the h function. This uses Manhattan distance
    dRow = abs(node.coords[0]-end.coords[0])
    dCol = abs(node.coords[1]-end.coords[1])
    return dRow + dCol


def reconstructPath(cameFrom, current: Node):
    totalPath = []
    while current in cameFrom.keys():
        current = cameFrom[current]
        totalPath.append(current)
    return totalPath[::-1]


def aStar(start: Node, end: Node, grid) -> int:
    openSet = deque()
    visited = set()
    openSet.append(start)

    cameFrom = {}

    gscore = {node: INFINITY for row in grid for node in row}
    gscore[start] = 0

    fscore = {node: INFINITY for row in grid for node in row}
    fscore[start] = heuristic(start, end)

    while len(openSet) > 0:
        current = openSet.popleft()
        visited.add(current)

        if current.coords == end.coords:
            return len(reconstructPath(cameFrom, current))

        for neighbour in findNeighbours(current, grid):
            gscoreGuess = gscore[current] + 1
            if gscoreGuess < gscore[neighbour]:
                cameFrom[neighbour] = current
                gscore[neighbour] = gscoreGuess
                fscore[neighbour] = gscoreGuess + heuristic(neighbour, end)
            if neighbour not in openSet and neighbour not in visited:
                openSet.append(neighbour)

    # no soluition
    return -1


def part1(grid: list[list[Node]], startNode: Node, endNode: Node):
    return aStar(startNode, endNode, grid)


def part2(grid: list[list[Node]], endNode: Node):
    startNodes = deque()
    for row in grid:
        for node in row:
            if node.elevation == 'S' or node.elevation == 'a':
                startNodes.append(node)

    best = INFINITY
    for start in startNodes:
        steps = aStar(start, endNode, grid)
        if steps < best and steps != -1:
            best = steps

    return best


def initializeGrid(input_arr):
    grid: list[list[Node]] = []
    startNode: Node = Node('a', (0, 0), [])
    endNode: Node = Node('z', (0, 0), [])
    for rowNum, line in enumerate(input_arr):
        row: list[Node] = []
        for colNum, char in enumerate(line):
            if char == "S":
                startNode.setCoords(rowNum, colNum)
                row.append(startNode)
            elif char == "E":
                endNode.setCoords(rowNum, colNum)
                row.append(endNode)
            else:
                row.append(Node(char, (rowNum, colNum), []))
            colNum += 1
        grid.append(row)
        rowNum += 1

    return grid, startNode, endNode


if __name__ == "__main__":
    with open("../../input/2022/day12.txt") as f:
        input_arr = f.read().split('\n')

        grid, startNode, endNode = initializeGrid(input_arr)
        print(f"part 1: {part1(grid, startNode, endNode)}")
        print(f"part 2: {part2(grid, endNode)}")
