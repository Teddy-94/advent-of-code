with open("..\..\input\day8.txt") as f:
    d = f.read()


def part1():
    count = 0
    for line in d.splitlines():
        _, output = line.split("|")
        output = output.split()
        # part1
        for s in output:
            if len(s) in {2, 3, 4, 7}:
                count += 1

    return(count)


def part2():
    keyForUnknownNumbers = [
        "abcefg",   # 0
        "acdeg",    # 2
        "acdfg",    # 3
        "abdfg",    # 5
        "abdefg",   # 6
        "abcdfg",   # 9
    ]

    solved = {

    }

    for line in d.splitlines():
        # for each line we want to first find the known digits, those with unique numbers of sections in use (s.len)
        # Then we use the known digits to find the rest of the digits.
        # Diffing the s with the known digits lets us find the remaining digits
        #
        input, output = line.split("|")
        output = [''.join(sorted(s)) for s in output.split()]
        input = [''.join(sorted(s)) for s in input.split()]
        digits = {*input, *output}
        
        
    def findKnown(s):
        if len(s) == 2:
            solved[1] = s
        elif len(s) == 3:
            solved[7] = s
        elif len(s) == 4:
            solved[4] = s
        elif len(s) == 7:
            solved[8] = s

    def findLengh6(s):
        # if s len == 6 then the digit is either 0, 6, or 9

        # 8 completely contains 0 <- finds 0

        # 9 completely contains 4 / 1 /  finds 9
 
        # else = 6 
        # 6 does not completely cover 1, 4, or 7 finds 6
        return

    def findLengh5(s):
        # if s len == 6 then the digit is either 2, 3, or 5

        # 3 completely contains 1 / 7 finds 3
        # 
        # 5 is one line different from 4
        # whereas 2 is two lines different from 4
        # if line diff 5 & 4 == 1, digit = 5 finds 5 

        # else 2 finds 2

        return

    return input, output, digits


print(part1(), part2())
