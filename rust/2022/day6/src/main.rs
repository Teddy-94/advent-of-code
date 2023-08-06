use std::collections::VecDeque;

fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");

    let signal_vec: Vec<char> = input.chars().collect::<Vec<char>>();

    println!("part 1 {}\npart 2 {}", part_1(&signal_vec), part_2(&signal_vec))
}

fn part_1(signal_vec: &Vec<char>) -> usize {
    return message_checker(4, signal_vec);
}

fn part_2(signal_vec: &Vec<char>) -> usize {
    return message_checker(14, signal_vec);
}

fn message_checker(message_len: usize, list_to_check: &Vec<char>) -> usize {
    let mut x = 0;
    while x < list_to_check.len() {
        // make a 4 char window
        let mut queue: VecDeque<char> = VecDeque::with_capacity(message_len);
        for i in 0..message_len {
            queue.push_back(list_to_check[x + i])
        }
        // check for dupes in window
        let mut count = 0;
        let mut unique_count = 0;
        for c in &queue {
            for each in &queue {
                if c == each {
                    count += 1;
                }
            }
            // char is not unique
            if count > 1 {
            } else {
                unique_count += 1;
            }
            count = 0;
        }
        if unique_count == message_len {
            println!("i think we got a signal on char {:?}", x + message_len);
            return x + message_len;
        }
        x += 1;
    }
    println!("there doesn't seem to be any messages here");
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");

        let chars: Vec<char> = input.chars().collect::<Vec<char>>();
        assert_eq!(part_1(&chars), 5);
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");

        let chars: Vec<char> = input.chars().collect::<Vec<char>>();
        assert_eq!(part_2(&chars), 23);
    }
}
