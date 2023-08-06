fn main() {
    let input = std::fs::read_to_string("../../input/day2.txt").expect("can't read file");

    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}

fn part_a(input: &String) -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for line in input.trim().lines() {
        let mut parts = line.split(" ");

        let direction: &str = parts.next().unwrap();
        let value: i32 = parts.next().unwrap().parse().unwrap();

        match direction {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => {}
        }
    }
    let result: i32 = horizontal * depth;
    return result;
}

fn part_b(input: &String) -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for line in input.trim().lines() {
        let mut parts = line.split(" ");

        let direction: &str = parts.next().unwrap();
        let value: i32 = parts.next().unwrap().parse().unwrap();

        match direction {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "down" => {
                aim += value;
            }
            "up" => {
                aim -= value;
            }
            _ => {}
        }
    }
    let result: i32 = horizontal * depth;
    return result;
}
