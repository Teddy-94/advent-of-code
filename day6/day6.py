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