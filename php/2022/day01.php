<?php

$input = file_get_contents("../../input/2022/day1.txt");

$inputArray = explode("\n", $input);
$inputIntArray = array_map(
    function ($str) {
        return $str === "" ? $str : (int)$str;
    },
    $inputArray
);

$elfs = [];
$currentElf = [];
$elfSums = [];

foreach ($inputIntArray as $cal) {
    if (!is_int($cal)) {
        if (!empty($currentElf)) {
            $elfs[] = $currentElf;
        }
        $currentElf = [];
    } else {
        $currentElf[] = $cal;
    };
}


if (!empty($currentElf)) {
    $elfs[] = $currentElf;
}

foreach ($elfs as $elf) {
    $calorieCount = 0;
    foreach ($elf as $calorie) {
        $calorieCount += $calorie;
    }
    $elfSums[] = $calorieCount;
}

rsort($elfSums);

// Part 1
echo($elfSums[0]);
echo("\n");

// Part 2
$top3 = $elfSums[0] + $elfSums[1] + $elfSums[2];
echo($top3);
echo("\n");
