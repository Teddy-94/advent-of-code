# part 1
lines = []
for d in open(".\input3.txt"):
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