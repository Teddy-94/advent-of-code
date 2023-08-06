with open("..\input\day4.txt", "r") as f:
    first, *rest = f.read().split("\n\n")
    # parse draws
    draws = [int(x) for x in first.strip().split(",")]
    # parse boards
    boards = [[[int(col) for col in row.split()]
               for row in board.split("\n")] for board in rest]


def markBoard(draw, board):
    for row in board:
        for i in range(0, len(row)):
            if row[i] == draw:
                row[i] = "x"


def sumBoard(board):
    sum = 0
    for row in board:
        for n in row:
            if n != "x":
                sum += n
    return sum


def checkWinner(board):
    win = False
    # rows
    for row in board:
        win = all(e in ["x"] for e in row)
        if win:
            return win
    # columns
    for i in range(0, 5):
        win = all(e in ["x"] for e in [row[i] for row in board])
        if win:
            return win

    return win


def part1():
    for draw in draws:
        for board in boards:
            markBoard(draw, board)

            if checkWinner(board):
                return (sumBoard(board)*draw)


def part2():
    winners = []
    winningDraw = None
    for draw in draws:
        for board in boards:
            if board not in winners:
                markBoard(draw, board)

                if checkWinner(board):
                    winners.append(board)
                    winningDraw = draw
    return (sumBoard(winners[-1])*winningDraw)


print(f"first winner score: {part1()}, last winner score: {part2()}")
