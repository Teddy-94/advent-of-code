use std::collections::HashSet;

fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();

    println!("part 1 {}\npart 2 {}", part_1(&row), part_2(&row))
}

#[allow(unused_variables)]
fn part_1(row: &Vec<&str>) -> usize {
    let mut tail_pos: (i32, i32) = (0, 0);
    let mut head_pos: (i32, i32) = (0, 0);
    let mut prev_head_pos: (i32, i32) = (0, 0);
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    fn check_tail_dist(head_pos: &(i32, i32), tail_pos: &(i32, i32)) -> bool {
        let y_diff = (head_pos.0 - tail_pos.0).abs();
        let x_diff = (head_pos.1 - tail_pos.1).abs();
        println!(
            "head position is {:?} and tail position is {:?}, Y diff is {:?}, X diff is {:?}",
            head_pos, tail_pos, y_diff, x_diff
        );

        if y_diff > 1 {
            return true;
        } else if x_diff > 1 {
            return true;
        } else {
            return false;
        }
    }

    tail_visited.insert(tail_pos);

    for r in row {
        let (direction, dist) = r.split_once(' ').unwrap();
        let dist = dist.parse::<i32>().unwrap();

        println!("direction {:?} and distance {:?}", direction, dist);
        println!("{:?}", r);

        match direction {
            "U" => {
                println!("direction is U");
                for n in 0..dist {
                    prev_head_pos = head_pos;
                    head_pos.0 += 1;
                    if !check_tail_dist(&head_pos, &tail_pos) {
                    } else {
                        println!("tail moved to {:?}", prev_head_pos);
                        tail_pos = prev_head_pos;
                        tail_visited.insert(tail_pos);
                    };
                }
            }
            "D" => {
                println!("direction is D");
                for n in 0..dist {
                    prev_head_pos = head_pos;
                    head_pos.0 -= 1;
                    if !check_tail_dist(&head_pos, &tail_pos) {
                    } else {
                        println!("tail moved to {:?}", prev_head_pos);
                        tail_pos = prev_head_pos;
                        tail_visited.insert(tail_pos);
                    };
                }
            }
            "L" => {
                println!("direction is L");
                for n in 0..dist {
                    prev_head_pos = head_pos;
                    head_pos.1 -= 1;
                    if !check_tail_dist(&head_pos, &tail_pos) {
                    } else {
                        println!("tail moved to {:?}", prev_head_pos);
                        tail_pos = prev_head_pos;
                        tail_visited.insert(tail_pos);
                    };
                }
            }
            "R" => {
                println!("direction is R");
                for n in 0..dist {
                    prev_head_pos = head_pos;
                    head_pos.1 += 1;
                    if !check_tail_dist(&head_pos, &tail_pos) {
                    } else {
                        println!("tail moved to {:?}", prev_head_pos);
                        tail_pos = prev_head_pos;
                        tail_visited.insert(tail_pos);
                    };
                }
            }
            _ => println!("dang!"),
        }
    }
    return tail_visited.len();
}

#[allow(unused_variables)]
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
        assert_eq!(part_1(&row), 13);
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
        assert_eq!(part_2(&row), 0);
    }
}
