fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();

    println!(
        "part 1 {}{}part 2 {}",
        part_1(&row),
        "\n",
        part_2(&row)
    )
}

fn part_1(row: &Vec<&str>) -> i32 {
    return 0;
}

fn part_2(row: &Vec<&str>) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
        assert_eq!(part_1(&row), 0);
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
        assert_eq!(part_2(&row), 0);
    }
}
