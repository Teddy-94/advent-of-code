position = depth = 0
for line in open (".\input2.txt"):
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

for line in open (".\input2.txt"):

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