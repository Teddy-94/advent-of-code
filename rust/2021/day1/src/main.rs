fn main() {
    let input_path = format!("../../../input/2021/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let nums: Vec<i32> = input
        .trim()
        .lines()
        .map(|n| n.parse::<i32>().expect("some error"))
        .collect();

    println!("{}", part_a(&nums));
    println!("{}", part_b(&nums));
}

fn part_a(nums: &Vec<i32>) -> i32 {
    let mut count: i32 = 0;

    for i in 0..nums.len() - 1 {
        if nums[i] < nums[i + 1] {
            count += 1;
        }
    }
    count
}

fn part_b(nums: &Vec<i32>) -> i32 {
    let mut count: i32 = 0;

    for i in 0..nums.len() - 3 {
        let a = nums[i] + nums[i + 1] + nums[i + 2];
        let b = nums[i + 1] + nums[i + 2] + nums[i + 3];
        if a < b {
            count += 1;
        }
    }
    count
}