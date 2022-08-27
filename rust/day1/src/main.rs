use std::fs;

fn main() {
    println!("Hello, world!");
}

fn read_data() -> Vec<usize>{
    let values = fs::read_to_string("/day1/day1.txt").expect("Couldn't read data");
    values
        .split('\n')
        .filter_map(|s|s.parse::<usize>().ok())
        .collect()
}