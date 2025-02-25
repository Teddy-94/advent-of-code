package main

import (
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("../../input/2024/day1.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	data := strings.Split(string(file), "\n")

	var left, right []float64

	for _, line := range data {
		a := strings.Fields(line)

		if len(a) == 2 {
			leftNum, err := strconv.ParseFloat(a[0], 64)
			if err != nil {
				fmt.Println(err)
				return
			}
			rightNum, err := strconv.ParseFloat(a[1], 64)
			if err != nil {
				fmt.Println(err)
				return
			}
			left = append(left, leftNum)
			right = append(right, rightNum)
		}
	}
	if len(left) != len(right) {
		fmt.Println("Length of left and right slices are not equal")
		return
	}
	slices.Sort(left)
	slices.Sort(right)

	part1 := part1(left, right)
	part2 := part2(left, right)
	fmt.Printf("Part 1: %.0f\n", part1)
	fmt.Printf("Part 2: %.0f\n", part2)
}

func part1(left, right []float64) float64 {
	var sum float64
	for i := range left {
		sum += math.Abs(left[i] - right[i])
	}

	return sum
}

func part2(left, right []float64) float64 {
	var sum float64
	for _, v := range left {
		count := 0
		for _, w := range right {
			if v == w {
				count++
			}
		}
		sum += float64(count) * v
	}

	return sum
}
