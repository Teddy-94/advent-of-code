from collections import defaultdict

points = defaultdict(int) 
points2 = defaultdict(int) 

with open ("..\..\input\day5.txt", "r") as f:
        d = f.read().strip().splitlines()

        #parse data
        for line in d:
            p1, p2 = line.split("->")
            x1, y1 = tuple(map(int, p1.split(",")))
            x2, y2 = tuple(map(int, p2.split(",")))

            dx = x2-x1
            dy = y2-y1

            for i in range(max(abs(dx), abs(dy)) + 1):
                x = x1 + (1 if dx > 0 else (-1 if dx < 0 else 0))*i
                y = y1 + (1 if dy > 0 else (-1 if dy < 0 else 0))*i                    

                if dx == 0 or dy == 0:
                    points[(x,y)] += 1
                points2[(x,y)] +=1
                
count = 0
for n in points:
    if points[n] > 1:
        count +=1
print(count)

count2 = 0
for n in points2:
    if points2[n] > 1:
        count2 +=1
print(count2)