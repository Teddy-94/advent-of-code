fn main() {
    let input = std::fs::read_to_string("../../input/day1.txt").expect("can't read file");
    let nums: Vec<i32> = input
        .trim()
        .lines()
        .map(|n| n.parse::<i32>().expect("some error"))
        .collect();

    part_a(&nums);
}

fn part_a(nums: &Vec<i32>) {
    
    let mut count:i32 = 0;

    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            count += 1;
        }
    }
    print!("{}", count.to_string());
}