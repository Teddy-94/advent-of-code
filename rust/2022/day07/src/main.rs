use std::collections::HashMap;

fn main() {
    let input_path = format!("../../../input/2022/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let row: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

    let mut dicts: HashMap<String, usize> = HashMap::new();
    println!(
        "part 1 {}\npart 2 {}",
        part_1(&row, &mut dicts),
        part_2(&mut dicts)
    )
}

fn part_1(row: &Vec<&str>, dicts: &mut HashMap<String, usize>) -> usize {
    let mut path = vec![];

    for row in row {
        let words = row.split(' ').collect::<Vec<_>>();

        if words[1] == "cd" {
            match words[2] {
                ".." => {
                    path.pop();
                }
                _ => {
                    path.push(words[2]);
                }
            }
        } else {
            match words[0] {
                "$" => continue,
                "dir" => continue,
                _ => {
                    let mut temp_path: String = "".to_string();
                    for s in &path {
                        temp_path.push_str(s);
                        let mut sum = words[0].parse::<usize>().unwrap();
                        if dicts.contains_key(&temp_path) {
                            let (_, v) = dicts.get_key_value(&temp_path).unwrap();
                            sum += v;
                            dicts.insert(temp_path.clone(), sum);
                        }
                        dicts.insert(temp_path.clone(), sum);
                    }
                }
            }
        }
    }

    let mut total: usize = 0;
    for d in dicts {
        if *d.1 <= 100000 {
            total += *d.1
        }
    }
    return total;
}

fn part_2(dicts: &mut HashMap<String, usize>) -> usize {
    let mut removal_candidates: Vec<usize> = vec![];

    let home_dir = &dicts.get_key_value("/").unwrap();
    let used_mem = 70000000 - *home_dir.1;
    let mem_to_free = 30000000 - used_mem;

    for d in &dicts.clone() {
        if *d.1 > mem_to_free {
            removal_candidates.push(*d.1)
        }
    }
    removal_candidates.sort();
    return *removal_candidates.first().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../../input/2022/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
        let mut dicts: HashMap<String, usize> = HashMap::new();
        assert_eq!(part_1(&row, &mut dicts), 95437);
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../../input/2022/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
        let mut dicts: HashMap<String, usize> = HashMap::new();
        part_1(&row, &mut dicts);
        assert_eq!(part_2(&mut dicts), 24933642);
    }
}
