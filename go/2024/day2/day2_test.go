package day2

import "testing"

func TestReportIsSafe(t *testing.T) {
	tests := []struct {
		report []string
		want   bool
	}{
		{[]string{"7", "6", "4", "2", "1"}, true},
		{[]string{"1", "2", "7", "8", "9"}, false},
		{[]string{"9", "7", "6", "2", "1"}, false},
		{[]string{"1", "3", "2", "4", "5"}, false},
		{[]string{"8", "6", "4", "4", "1"}, false},
		{[]string{"1", "3", "6", "7", "9"}, true},
	}

	for _, test := range tests {
		t.Run("", func(t *testing.T) {
			got, _ := reportIsSafe(test.report)
			if got != test.want {
				t.Errorf("reportIsSafe(%v) = %v; want %v", test.report, got, test.want)
			}
		})
	}
}
