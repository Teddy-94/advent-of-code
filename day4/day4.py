draws = None
board = []
boards = []

row = []
n = 0

for line in open (".\input4.txt", "r"):
    line = line.strip()

    #filter out the first line
    if draws is None:
        draws = [int(x) for x in line.split(",")]

    #parse the boards
    else: 
        if line:
            board.append([int(x) for x in line.split()])
        else:
            if board:
                boards.append(board)
                board = []
boards.append(board)

#for every number drawn
for draw in draws:
    #check if the number is on a board and mark it true
    for board in boards:
        for i in range(5):
            for j in range(5):
                if draw in board[i]:
                    board[i] = True
                    print(board[i])