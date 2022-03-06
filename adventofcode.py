def Day1():
    with open (".\inputs\input1.txt", "r") as f:
            d = f.read().strip().splitlines()
    depth_increases = 0
    for i in range(1, len(d)):
        if (d[i] > d[i-1]):
            depth_increases += 1
    print(depth_increases)

    depth_increases = 0
    for i in range(2, len(d)):
        if (d[i-3]+d[i-2]+d[i-1]<d[i-2]+d[i-1]+d[i]):
            depth_increases +=1
    print(depth_increases)

def Day2():
    position = depth = 0
    for line in open (".\inputs\input2.txt"):
        command, n = line.split(" ")
        n = int(n)
        
        if command == "up":
            depth -=n
        elif command == "down":
            depth +=n
        elif command == "forward":
            position +=n

    print (position*depth)

    position = depth = aim = 0

    for line in open (".\inputs\input2.txt"):
        command, n = line.split(" ")
        n = int(n)

        if command == "up":
            aim -= n
        elif command == "down":
            aim += n
        elif command == "forward":
            position += n
            depth += n*aim
    print (position*depth)

def Day3():
    # part 1
    lines = []
    for d in open(".\inputs\input3.txt"):
        lines.append(d.splitlines())

    count = [0] * len(lines[0])
    
    for line in lines:
        for i, char in enumerate(line):
            if char == "1":
                count[i] += 1
            else:
                count[i] -= 1  

    gamma = ""
    for i in count:
        if i <= 0:
            gamma+="0"
        else: gamma+="1"
    epsilon="".join(chr(ord(c)^1) for c in gamma)
    print(int(gamma,2)*int(epsilon,2))

    # part 2
    o=list(lines)
    co=list(lines)
    i=0
    while len(o) > 1 or len(co) > 1:

        if len(o) > 1:
            o0 = len([x for x in o if x[i]=="0"])
            o1 = len([x for x in o if x[i]=="1"])
            if o1 >= o0:
                o = [x for x in o if x[i]=="1"]
            else:
                o = [x for x in o if x[i]=="0"]
    
        if len(co) > 1:
            co0 = len([x for x in co if x[i]=="0"])
            co1 = len([x for x in co if x[i]=="1"])
            if co1 >= co0:
                co = [x for x in co if x[i]=="0"]
            else:
                co = [x for x in co if x[i]=="1"]
        i += 1

    print (int(o[0],2) * int(co[0],2))

def Day4():
    with open (".\input4.txt", "r") as f:
        draws = f.readline().split(",")
        boards = []

        print(draws)   
    return
    

def Day5():
    print("Hi")

def Day6():
    with open (".\inputs\input6.txt", "r") as f: 
        fish = list(map(int, f.read().split(",")))
        fishes =[0] * 9

        for i in fish:
            fishes[i] +=1

        for _ in range(256):
            zero = fishes[0]
            fishes = fishes[1:] + [zero]
            fishes[6] += zero
    print(sum(fishes))


def Day7():
    print("Hi")

def Day8():
    print("Hi")

def Day9():
    print("Hi")

def Day10():
    print("Hi")

def Day11():
    print("Hi")

def Day12():
    print("Hi")

def Day13():
    print("Hi")

def Day14():
    print("Hi")

def Day15():
    print("Hi")

def Day16():
    print("Hi")

def Day17():
    print("Hi")

def Day18():
    print("Hi")

def Day19():
    print("Hi")

def Day20():
    print("Hi")

def Day21():
    print("Hi")

def Day22():
    print("Hi")

def Day23():
    print("Hi")

def Day24():
    print("Hi")

def Day25():
    print("Hi")

if __name__ == "__main__":
    # Day1()
    # Day2()
    #Day3()
    # Day4()
    # Day5()
     Day6()
    # Day7()
    # Day8()
    # Day9()
    # Day10()
    # Day11()
    # Day12()
    # Day13()
    # Day14()
    # Day15()
    # Day16()
    # Day17()
    # Day18()
    # Day19()
    # Day20()
    # Day21()
    # Day22()
    # Day23()
    # Day24()
    # Day25()