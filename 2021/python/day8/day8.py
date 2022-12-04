with open("..\..\input\day8.txt") as f:
    data = [line for line in f.read().strip().splitlines()]

    inputs = [line.split(" | ")[0].split(" ") for line in data]
    inputs = [[''.join(sorted(s)) for s in input] for input in inputs]

    outputs = [line.split(" | ")[1].split(" ") for line in data]
    outputs = [[''.join(sorted(s)) for s in output] for output in outputs]


def part1(outputs):
    count = 0
    for output in outputs:
        for digit in output:
            if len(digit) in {2, 3, 4, 7}:
                count += 1

    return(count)


def part2(inputs, outputs):
    solved = {}
    total = 0

    for index, input in enumerate(inputs):

        # solve digits with unique numbers of active segments
        for digit in input:
            if len(digit) == 2:
                solved[1] = digit
            elif len(digit) == 4:
                solved[4] = digit
            elif len(digit) == 3:
                solved[7] = digit
            elif len(digit) == 7:
                solved[8] = digit

        # Find 0, 6, 9, all which have 6 segments active
        for digit in input:
            if len(digit) == 6:
                # 1 is completely covered by 0 and 9, but not 6
                if diff(solved[1], digit) == 1:
                    solved[6] = digit
                # 4 is completely covered by 9 but not 0
                elif diff(solved[4], digit) == 0:
                    solved[9] = digit
                # 0 is the only 6-len digit left
                else:
                    solved[0] = digit

        # Find 2, 3, 5, all which have 5 segments active
        for digit in input:
            if len(digit) == 5:
                # 3 completely covers 1
                if diff(solved[1], digit) == 0:
                    solved[3] = digit
                # 5 misses one section of 4, whereas 2 misses 2 sections
                elif diff(solved[4], digit) == 1:
                    solved[5] = digit
                # 0 is the only 6-len digit left
                else:
                    solved[2] = digit


        currentNumber = []
        for digit in outputs[index]:
            for key, value in solved.items():
                #print(digit, value)
                if digit == value:
                    currentNumber.append(str(key))
        currentNumber = int("".join(currentNumber))
        total = total+currentNumber

    return total


def diff(knownDigit, candidateDigit):
    diffCount = 0
    # Checks how many of the segments in the known digit are NOT in the candidate digit
    for char in knownDigit:
        if char not in candidateDigit:
            diffCount += 1
    return diffCount


print(
    part1(outputs),
    part2(inputs, outputs)
)
