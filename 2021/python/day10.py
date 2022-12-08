with open("..\input\day10.txt") as f:
    d = f.readlines()


def part1():
    score = {
        ")": 3,
        "]": 57,
        "}": 1197,
        ">": 25137
    }

    closingTag = {
        "(": ")",
        "[": "]",
        "{": "}",
        "<": ">",
    }

    total = 0

    for line in d:
        stack = []
        ok = True
        for char in line:
            if not ok:
                break
            for openingTag in closingTag:
                if char == openingTag:
                    stack.append(char)
                elif char == closingTag[openingTag]:
                    if openingTag == stack[-1]:
                        stack.pop()
                    else:
                        total = total + score[char]
                        ok = False
                        break
    return(total)


def part2():
    score = {
        ")": 1,
        "]": 2,
        "}": 3,
        ">": 4
    }

    closingTag = {
        "(": ")",
        "[": "]",
        "{": "}",
        "<": ">",
    }

    totalScore = []

    for line in d:
        lineScore = 0
        stack = []
        ok = True
        for idx, char in enumerate(line):
            if not ok:
                break
            for openingTag in closingTag:
                if char == openingTag:
                    stack.append(char)
                elif char == closingTag[openingTag]:
                    if openingTag == stack[-1]:
                        stack.pop()
                    else:
                        # this is a corrupted line
                        ok = False

            if ok and idx == len(line)-1:
                # this is an unfinished line, needs to be completed and scored

                for i in range(len(stack)):
                    for openingTag in closingTag:
                        if openingTag == stack[len(stack)-1-i]:
                            lineScore = lineScore*5 + \
                                score[closingTag[openingTag]]

                totalScore.append(lineScore)
    return(sorted(totalScore)[len(totalScore)//2])


print(part1(), part2(),)
