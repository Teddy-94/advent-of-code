// template
fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let elf_pairs: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();

    println!(
        "part 1 {}{}part 2 {}",
        part_1(&elf_pairs),
        "\n",
        part_2(&elf_pairs)
    )
}

fn part_1(elf_pairs: &Vec<&str>) -> i32 {
    let mut count: i32 = 0;
    for elfs in elf_pairs {
        let (elf_a, elf_b) = elfs.split_once(',').unwrap();
        let (a_start, a_end) = elf_a.split_once('-').unwrap();
        let (a_start, a_end): (i32, i32) = (a_start.parse().unwrap(), a_end.parse().unwrap());
        let (b_start, b_end) = elf_b.split_once('-').unwrap();
        let (b_start, b_end): (i32, i32) = (b_start.parse().unwrap(), b_end.parse().unwrap());

        if a_start >= b_start && a_end <= b_end || b_start >= a_start && b_end <= a_end {
            count += 1;
        }
    }

    return count;
}

fn part_2(elf_pairs: &Vec<&str>) -> i32 {
    let mut count: i32 = 0;
    for elfs in elf_pairs {
        let (elf_a, elf_b) = elfs.split_once(',').unwrap();
        let (a_start, a_end) = elf_a.split_once('-').unwrap();
        let (a_start, a_end): (i32, i32) = (a_start.parse().unwrap(), a_end.parse().unwrap());
        let (b_start, b_end) = elf_b.split_once('-').unwrap();
        let (b_start, b_end): (i32, i32) = (b_start.parse().unwrap(), b_end.parse().unwrap());

        if a_start >= b_start && a_start <= b_end
            || a_end >= b_start && a_end <= b_end
            || a_start >= b_start && a_end <= b_end
            || b_start >= a_start && b_end <= a_end
        {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
        assert_eq!(part_1(&row), 2);
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
        assert_eq!(part_2(&row), 4);
    }
}
