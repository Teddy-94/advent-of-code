fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let backpacks: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();

    println!("part 1 {}part 2 {}", part_1(&backpacks), part_2(&backpacks))
}

fn part_1(backpacks: &Vec<&str>) -> i32 {
    let mut incorrect_item_types = vec![];
    let mut priorities: i32 = 0;

    for backpack in backpacks {
        let (compartment_a, compartment_b) = backpack.split_at(backpack.len() / 2);
        for c in compartment_a.chars() {
            if compartment_b.contains(c) {
                incorrect_item_types.push(c);
                break;
            }
        }
    }

    for item in &incorrect_item_types {
        if item.is_lowercase() {
            priorities += -96 + *item as i32;
        }
        if item.is_ascii_uppercase() {
            priorities += -38 + *item as i32;
        }
    }
    return priorities;
}

fn part_2(backpacks: &Vec<&str>) -> i32 {
    let mut group_badges = vec![];
    let mut priorities: i32 = 0;

    for i in (2..backpacks.len()).step_by(3) {
        for c in backpacks[i].chars() {
            if backpacks[i - 1].contains(c) && backpacks[i - 2].contains(c) {
                group_badges.push(c);
                break;
            }
        }
    }
    for item in &group_badges {
        if item.is_lowercase() {
            priorities += -96 + *item as i32;
        }
        if item.is_ascii_uppercase() {
            priorities += -38 + *item as i32;
        }
    }

    return priorities;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let backpacks: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
        assert_eq!(part_1(&backpacks), 157);
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let backpacks: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
        assert_eq!(part_2(&backpacks), 70);
    }
}
