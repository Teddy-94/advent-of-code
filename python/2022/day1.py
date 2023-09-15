with open("../../input/2022/day1.txt") as f:
    input_arr = f.read().split('\n')

current_elf = []
elfs = []
elf_sums = []

for row in input_arr:
    if row == '':
        elfs.append(current_elf)
        current_elf = []
    else:
        row = int(row)
        current_elf.append(row)

elfs.append(current_elf)
current_elf = []

for elf in elfs:
    elf_sums.append(sum(elf))
sorted_elf_sums = sorted(elf_sums, reverse=True)

# Part 1
print(sorted_elf_sums[0])

# Part 2
print(sorted_elf_sums[0] + sorted_elf_sums[1] + sorted_elf_sums[2])
