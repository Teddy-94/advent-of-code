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
    with open (".\inputs\input2.txt", "r") as f:
            d = f.read().strip().split(" ")
    fwd = depth = aim = 0
    
    for i, d in enumerate(d):
        if d == "forward":
            fwd +=  f[i]
        if d == "up":
            depth += f[i]
        if d == "down":
            depth -= f[i]
        
    print(fwd*abs(depth))

    for i, d in enumerate(d):
        if d == "forward":
            fwd += f[i]
            depth += f[i]*aim
        if d == "up":
            aim += f[i]
        if d == "down":
            aim -= f[i]
        
    print(fwd*abs(depth))

def Day3():
    # part 1
    d = []
    count = [0,0,0,0,0,0,0,0,0,0,0,0]
    
    for line in d:
        for i, char in enumerate(line):
            if char == "1":
                count[i] += 1
            else:
                count[i] -= 1  

    g = ""
    for i in count:
        if i <= 0:
            g+="0"
        else: g+="1"
    e="".join(chr(ord(c)^1) for c in g)
    
    print(int(g,2)*int(e,2))

    # part 2
    o=d
    co=d
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
    with open (r"D:\Prog\adventofcode\input6.txt", "r") as f: 
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
     Day2()
    # Day3()
    # Day4()
    # Day5()
    # Day6()
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