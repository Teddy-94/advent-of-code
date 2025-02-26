package day2

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func Run() {
	file, err := os.ReadFile("../../input/2024/day2.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	data := strings.Split(string(file), "\n")

	reports := make([][]string, len(data))
	for i, v := range data {
		report := strings.Fields(v)
		reports[i] = report
	}

	var safeReports int
	for _, report := range reports {
		safe, err := reportIsSafe(report)
		if err != nil {
			fmt.Println(err)
			return
		}
		if safe {
			safeReports++
		}
	}
	fmt.Println(safeReports)
}

func reportIsSafe(report []string) (bool, error) {
	reportInt := make([]float64, len(report))
	for i, v := range report {
		n, err := strconv.ParseFloat(v, 64)
		if err != nil {
			return false, err
		}
		reportInt[i] = n
	}

	increasing := reportInt[0] < reportInt[1]
	for i := 1; i < len(reportInt); i++ {
		if increasing && reportInt[i-1] > reportInt[i] {
			return false, nil
		}
		if !increasing && reportInt[i-1] < reportInt[i] {
			return false, nil
		}
		if !safeDiff(reportInt[i-1], reportInt[i]) {
			return false, nil
		}
	}
	return true, nil
}

func safeDiff(a, b float64) bool {
	if a == b {
		return false
	}
	if math.Abs(a-b) > 3 {
		return false
	}
	return true
}
