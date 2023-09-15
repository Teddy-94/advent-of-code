with open("../../input/2022/day2.txt") as f:
    input_arr = f.read().strip().split('\n')


def check_winner(opp, you):
    if you == "X":
        if opp == "A":
            return "draw"
        if opp == "B":
            return "lose"
        else:
            return "win"
    if you == "Y":
        if opp == "A":
            return "win"
        if opp == "B":
            return "draw"
        else:
            return "lose"
    if you == "Z":
        if opp == "A":
            return "lose"
        if opp == "B":
            return "win"
        else:
            return "draw"
    else:
        return ""


def part1():
    # A rock X, B paper Y, C scissor Z
    score = {
        "X": 1,
        "Y": 2,
        "Z": 3,
        "lose": 0,
        "draw": 3,
        "win": 6,
    }

    total_score = 0
    for line in input_arr:
        opp, you = line.split(' ')
        outcome = check_winner(opp, you)
        total_score += score[you]
        total_score += score[outcome]

    return total_score


def part2():
    # A rock, B paper, C scissors
    # X lose, Y draw, Z win

    score = {
        "A": 1,
        "B": 2,
        "C": 3,
        "X": 0,
        "Y": 3,
        "Z": 6,
    }

    def strategy(opp, outcome):
        if (opp == "A"):
            if (outcome == "X"):
                return "C"
            if (outcome == "Y"):
                return "A"
            else:
                return "B"
        if (opp == "B"):
            if (outcome == "X"):
                return "A"
            if (outcome == "Y"):
                return "B"
            else:
                return "C"

        if (opp == "C"):
            if (outcome == "X"):
                return "B"
            if (outcome == "Y"):
                return "C"
            else:
                return "A"
        else:
            return ""

    total_score = 0

    for line in input_arr:
        opp, outcome = line.split(' ')
        you = strategy(opp, outcome)
        total_score += score[you]
        total_score += score[outcome]

    return total_score


print(part1(), part2())
