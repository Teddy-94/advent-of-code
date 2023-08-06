use std::collections::HashSet;

fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();

    println!("part 1 {}\npart 2 {}", part_1(&row), part_2(&row))
}

fn part_1(row: &Vec<&str>) -> usize {
    let mut tail_pos: (i32, i32) = (0, 0);
    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    fn decide_move(pos_1: &(i32, i32), pos_2: &(i32, i32)) -> (i32, i32, bool) {
        let dy = pos_1.0 - pos_2.0;
        let dx = pos_1.1 - pos_2.1;
        let mut to_move = (0, 0, false);

        if dy.abs() > 1 || dx.abs() > 1 {
            to_move.2 = true
        }

        if dy < 0 {
            to_move.0 = -1
        } else if dy > 0 {
            to_move.0 = 1
        }
        if dx < 0 {
            to_move.1 = -1
        } else if dx > 0 {
            to_move.1 = 1
        }
        return to_move;
    }

    tail_visited.insert(tail_pos);

    for instruction in row {
        let (direction, dist) = instruction.split_once(' ').unwrap();
        let dist = dist.parse::<i32>().unwrap();

        for _ in 0..dist {
            match direction {
                "U" => head_pos.0 += 1,
                "D" => head_pos.0 -= 1,
                "L" => head_pos.1 -= 1,
                "R" => head_pos.1 += 1,
                _ => println!("dang!"),
            }
            let (move_y, move_x, move_bool) = decide_move(&head_pos, &tail_pos);
            if move_bool {
                tail_pos.0 += move_y;
                tail_pos.1 += move_x;
            } else {
                // no need to move
            };
            tail_visited.insert(tail_pos);
        }
    }

    return tail_visited.len();
}

fn part_2(row: &Vec<&str>) -> usize {
    let mut rope: [(i32, i32); 10] = [
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    fn decide_move(pos_1: &(i32, i32), pos_2: &(i32, i32)) -> (i32, i32, bool) {
        let dy = pos_1.0 - pos_2.0;
        let dx = pos_1.1 - pos_2.1;
        let mut to_move = (0, 0, false);

        if dy.abs() > 1 || dx.abs() > 1 {
            to_move.2 = true
        }

        if dy < 0 {
            to_move.0 = -1
        } else if dy > 0 {
            to_move.0 = 1
        }
        if dx < 0 {
            to_move.1 = -1
        } else if dx > 0 {
            to_move.1 = 1
        }
        return to_move;
    }

    tail_visited.insert(rope[rope.len() - 1]);

    for instruction in row {
        let (direction, dist) = instruction.split_once(' ').unwrap();
        let dist = dist.parse::<i32>().unwrap();

        for _ in 0..dist {
            // only head moves according to the instructions. rest of rope moves according to decide_move
            match direction {
                "U" => rope[0].0 += 1,
                "D" => rope[0].0 -= 1,
                "L" => rope[0].1 -= 1,
                "R" => rope[0].1 += 1,
                _ => println!("dang!"),
            }
            for i in 1..rope.len() {
                let (move_y, move_x, move_bool) = decide_move(&rope[i - 1], &rope[i]);
                if move_bool {
                    rope[i].0 += move_y;
                    rope[i].1 += move_x;
                    if i == rope.len() - 1 {
                        // if we moved the tail, add it to the pos_visited
                        tail_visited.insert(rope[i]);
                    }
                } else {
                    // no need to move
                };
            }
        }
    }

    return tail_visited.len();
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
        let input_path = format!("../../input/{}_test2.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
        assert_eq!(part_2(&row), 36);
    }
}
