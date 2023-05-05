import { readFileSync } from 'fs';

const input = readFileSync('../input/day1.txt', 'utf-8');
const input_arr = input.split('\r\n').map(x => { return parseInt(x, 10) })

const elfs: number[][] = [];
let currentElf: number[] = [];
let elfsums: number[] = [];

input_arr.forEach(line => {
    if (isNaN(line)) {
        elfs.push(currentElf)
        currentElf = [];
    } else {
        currentElf.push(line)
    }
});

elfs.push(currentElf)
currentElf = []


elfs.forEach(elf => {
    let calorieCount = 0;
    elf.forEach(num => {
        calorieCount += num;
    })
    elfsums.push(calorieCount)
})

const elfsums_sorted = elfsums.sort((a, b) => b - a)

// part 1
console.log(elfsums_sorted[0]);

// part 2
console.log(elfsums_sorted.slice(0, 3).reduce((a, b) => a + b));


