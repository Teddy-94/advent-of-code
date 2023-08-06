fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let (crates, lines) = &input.split_once("\r\n\r\n").unwrap();
    let instructions_vec = lines.split("\r\n").collect::<Vec<&str>>();

    let mut crate_stacks: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    for row in crates.split("\r\n") {
        for (i, c) in row.chars().enumerate() {
            if c.is_alphabetic() {
                if i == 1 {
                    crate_stacks[0].push(c)
                }
                if i == 5 {
                    crate_stacks[1].push(c)
                }
                if i == 9 {
                    crate_stacks[2].push(c)
                }
                if i == 13 {
                    crate_stacks[3].push(c)
                }
                if i == 17 {
                    crate_stacks[4].push(c)
                }
                if i == 21 {
                    crate_stacks[5].push(c)
                }
                if i == 25 {
                    crate_stacks[6].push(c)
                }
                if i == 29 {
                    crate_stacks[7].push(c)
                }
                if i == 33 {
                    crate_stacks[8].push(c)
                }
            }
        }
    }

    println!(
        "part 1 {}\npart 2 {}",
        part_1(crate_stacks.clone(), instructions_vec.clone()),
        part_2(crate_stacks, instructions_vec)
    )
}

fn part_1(mut crate_stacks: Vec<Vec<char>>, instructions_vec: Vec<&str>) -> String {
    for i in 0..crate_stacks.len() {
        crate_stacks[i].reverse()
    }

    for inst in &instructions_vec {
        let instructions = inst.split(" ").collect::<Vec<&str>>();
        let amount = instructions[1].parse::<usize>().unwrap();
        let from = instructions[3].parse::<usize>().unwrap() - 1;
        let to = instructions[5].parse::<usize>().unwrap() - 1;

        let mut counter: usize = 0;
        while counter < amount {
            println!(
                "the instruction was move {:?} from stack {} {:?} to stack {} {:?}",
                amount, from, crate_stacks[from], to, crate_stacks[to]
            );
            let c = crate_stacks[from].pop().unwrap();
            crate_stacks[to].push(c);
            counter += 1;
            println!(
                "i took {:?} from {:?} to {:?}",
                c, crate_stacks[from], crate_stacks[to]
            );
        }
    }

    let mut final_string = "".to_string();

    println!("{:?}", crate_stacks.len());
    for i in 0..crate_stacks.len() {
        println!("{:?}", &crate_stacks[i]);
        final_string += crate_stacks[i].last().unwrap().to_string().as_str();
    }

    return final_string;
}

fn part_2(mut crate_stacks: Vec<Vec<char>>, instructions_vec: Vec<&str>) -> String {
    for i in 0..crate_stacks.len() {
        crate_stacks[i].reverse()
    }

    for inst in &instructions_vec {
        let instructions = inst.split(" ").collect::<Vec<&str>>();
        let amount = instructions[1].parse::<usize>().unwrap();
        let from = instructions[3].parse::<usize>().unwrap() - 1;
        let to = instructions[5].parse::<usize>().unwrap() - 1;

        let mut counter: usize = 0;
        let mut moving_stack: Vec<char> = vec![];
        while counter < amount {
            println!(
                "the instruction was move {:?} from stack {} {:?} to stack {} {:?}",
                amount, from, crate_stacks[from], to, crate_stacks[to]
            );
            let c = crate_stacks[from].pop().unwrap();
            moving_stack.push(c);
            counter += 1;
            println!(
                "i took {:?} from {:?} to {:?}",
                c, crate_stacks[from], crate_stacks[to]
            );
        }
        moving_stack.reverse();
        for item in moving_stack {
            crate_stacks[to].push(item);
        }
    }

    let mut final_string = "".to_string();

    println!("{:?}", crate_stacks.len());
    for i in 0..crate_stacks.len() {
        println!("{:?}", &crate_stacks[i]);
        final_string += crate_stacks[i].last().unwrap().to_string().as_str();
    }
    return final_string;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let (crates, lines) = &input.split_once("\r\n\r\n").unwrap();
        let instructions_vec = lines.split("\r\n").collect::<Vec<&str>>();

        let mut crate_stacks: Vec<Vec<char>> = vec![vec![], vec![], vec![]];

        for row in crates.split("\r\n") {
            for (i, c) in row.chars().enumerate() {
                if c.is_alphabetic() {
                    if i == 1 {
                        crate_stacks[0].push(c)
                    }
                    if i == 5 {
                        crate_stacks[1].push(c)
                    }
                    if i == 9 {
                        crate_stacks[2].push(c)
                    }
                }
            }
        }

        assert_eq!(part_1(crate_stacks, instructions_vec), "CMZ");
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let (crates, lines) = &input.split_once("\r\n\r\n").unwrap();
        let instructions_vec = lines.split("\r\n").collect::<Vec<&str>>();

        let mut crate_stacks: Vec<Vec<char>> = vec![vec![], vec![], vec![]];

        for row in crates.split("\r\n") {
            for (i, c) in row.chars().enumerate() {
                if c.is_alphabetic() {
                    if i == 1 {
                        crate_stacks[0].push(c)
                    }
                    if i == 5 {
                        crate_stacks[1].push(c)
                    }
                    if i == 9 {
                        crate_stacks[2].push(c)
                    }
                }
            }
        }

        assert_eq!(part_2(crate_stacks, instructions_vec), "MCD");
    }
}
