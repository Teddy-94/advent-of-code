fn main() {
    let input_path = format!("../../../input/2022/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let row: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

    println!("part 1 {}", part_1_and_2(&row))
}

fn part_1_and_2(row: &Vec<&str>) -> i32 {
    let mut current_cycle = 0;
    let mut x_register = 1;
    let mut sum_signal_strenght = 0;

    for r in row {
        current_cycle += 1;
        sum_signal_strenght += update_signal_strength(&x_register, &current_cycle);

        draw_crt(&current_cycle, &x_register);

        if r.contains("addx") {
            let (_, b) = r.split_once(' ').unwrap();
            let value: i32 = b.parse().unwrap();

            current_cycle += 1;
            draw_crt(&current_cycle, &x_register);

            sum_signal_strenght += update_signal_strength(&x_register, &current_cycle);
            x_register += value;
        }

        fn update_signal_strength(x_register: &i32, current_cycle: &i32) -> i32 {
            if [20, 60, 100, 140, 180, 220].contains(&current_cycle) {
                let signal_strength_delta = x_register * current_cycle;
                return signal_strength_delta;
            }
            return 0;
        }

        fn draw_crt(current_cycle: &i32, x_register: &i32) {
            let sprite_pos = [x_register - 1, x_register + 0, x_register + 1];
            let current_cycle_mod_40 = current_cycle % 40 - 1;

            if current_cycle_mod_40 == 0 {
                println!();
            }
            if sprite_pos.contains(&current_cycle_mod_40) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
    return sum_signal_strenght;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../../input/2022/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
        assert_eq!(part_1_and_2(&row), 13140);
    }
}
