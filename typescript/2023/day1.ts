import { readFileSync } from 'fs';

const input: string = readFileSync('../../input/2023/day01', 'utf-8');
const inputArr = input.trim().split("\n");

function part1(inputArr: Array<string>) {
  const inputArrArr = inputArr.map((x) => x.split(""))

  const numArr = inputArrArr.map((x) => x.filter((y) => parseInt(y)))

  const res = numArr.map((element) => {
    if (element.length > 1) {
      return parseInt(element[0]) * 10 + parseInt(element[element.length - 1])
    } else {
      return parseInt(element[0]) * 10 + parseInt(element[0])
    }
  })


  return res.reduce((prev, curr) => prev + curr)

  console.log(inputArr[1], numArr[1], res[1])
}

function part2(inputArr: Array<string>) {
  const numbersInText = {
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9,
    'zero': 0,

  }
}

console.log(part1(inputArr))
