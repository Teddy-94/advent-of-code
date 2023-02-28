fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();

    println!("part 1 {}\npart 2 {}", part_1(&row), part_2(&row))
}

fn part_1(row: &Vec<&str>) -> i32 {
    let (mut instruction, mut value) = ("", 0);
    let mut cycle = 0;
    let mut x_register = 0;
    let mut sum_signal_strenght = 0;
    fn signal_strenght_modulator(x_register: &i32, current_cycle: &i32) -> i32 {
        let signal_strenght = x_register * current_cycle;
        return signal_strenght;
    }

    fn increase_cycle(
        count: i32,
        mut current_cycle: i32,
        x_register: i32,
        mut sum_signal_strenght: i32,
    ) -> usize {
        for _ in 0..count {
            current_cycle += 1;
            if current_cycle == 20 {
                sum_signal_strenght += signal_strenght_modulator(&x_register, &current_cycle)
            }
        }
        return 1;
    }

    for r in row {
        if r.contains("addx") {
            let (a, b) = r.split_once(' ').unwrap();
            instruction = a;
            value = b.parse().unwrap();
        }
        match instruction {
            "noop" => {
                cycle += 1;
            }
            "addx" => {
                cycle += 2;
                //x_register += value;
            }
            _ => println!("Whoah!"),
        }

        println!(" {:?} {:?} ", r, value);
    }
    println!(" cycle {:?} register {:?} ", cycle, x_register);
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
        assert_eq!(part_1(&row), 13140);
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
        assert_eq!(part_2(&row), 0);
    }
}
