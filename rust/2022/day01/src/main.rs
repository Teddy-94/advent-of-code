fn main() {
    let input_path = format!("../../../input/2022/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let elfs: Vec<Vec<i32>> = input
        .split("\n\n")
        .map(|elf| -> Vec<i32> {
            elf.split("\n")
                .filter_map(|calorie| calorie.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut elfsums: Vec<i32> = vec![];
    println!(
        "part 1 {}\npart 2 {}",
        part_1(&elfs, &mut elfsums),
        part_2(&mut elfsums)
    );
}

fn part_1(elfs: &Vec<Vec<i32>>, elfsums: &mut Vec<i32>) -> i32 {
    for elf in elfs.iter() {
        let mut sum: i32 = 0;
        for calorie in elf.iter() {
            sum += calorie;
        }

        elfsums.push(sum)
    }
    let max: i32 = *elfsums.iter().max().unwrap();
    return max;
}

fn part_2(elfsums: &mut Vec<i32>) -> i32 {
    elfsums.sort();
    let lenght = elfsums.len();
    let sum_top_three: i32 = elfsums[lenght - 1] + elfsums[lenght - 2] + elfsums[lenght - 3];

    return sum_top_three;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../../input/2022/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let elfs: Vec<Vec<i32>> = input
            .split("\n\n")
            .map(|elf| -> Vec<i32> {
                elf.split("\n")
                    .filter_map(|calorie| calorie.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let mut elfsums: Vec<i32> = vec![];
        println!("{}", part_1(&elfs, &mut elfsums));
        assert_eq!(part_1(&elfs, &mut elfsums), 24000);
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../../input/2022/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let elfs: Vec<Vec<i32>> = input
            .split("\n\n")
            .map(|elf| -> Vec<i32> {
                elf.split("\n")
                    .filter_map(|calorie| calorie.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let mut elfsums: Vec<i32> = vec![];

        part_1(&elfs, &mut elfsums);
        println!("{}", part_2(&mut elfsums));
        assert_eq!(part_2(&mut elfsums), 45000);
    }
}
