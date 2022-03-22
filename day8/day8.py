with open (".\input8.txt") as f:
    d = f.read()
    count = 0
    
    for line in d.splitlines():
        input, output = line.split("|")
        input = input.split() 
        output = output.split()
        # part1
        for s in output:
            if len(s) in {2, 3, 4, 7}:
                count +=1
        
    print(count)

    
"""
0: abcefg
1: cf
2: acdeg
3: acdfg
4: bcdf
5: abdfg
6: abdefg
7: acf
8: abcdefg
9: abcdfg
"""