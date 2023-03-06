use std::cell::RefCell;

struct Monkey {
    items: RefCell<Vec<i64>>,
    operation: char,
    operation_value: i64,
    divisible_test: i64,
    throw_to_monkey_if_true: usize,
    throw_to_monkey_if_false: usize,
    item_inspection_count: RefCell<i32>,
}

fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let monkey_vec = get_monkey_vec(input);

    println!(
        "part 1 {}\npart 2 {}",
        part_1(&monkey_vec),
        part_2(&monkey_vec)
    )
}

fn part_1(monkey_vec: &Vec<Monkey>) -> i64 {
    monkey_biz(20, &monkey_vec, false);
    return get_monkey_biz_level(&monkey_vec);
}

fn part_2(monkey_vec: &Vec<Monkey>) -> i64 {
    monkey_biz(1000, &monkey_vec, true);
    return get_monkey_biz_level(&monkey_vec);
}

fn get_monkey_vec(input: String) -> Vec<Monkey> {
    let monkey_input: Vec<&str> = input.split("\r\n\r\n").collect::<Vec<&str>>();
    let mut monkeys: Vec<Monkey> = vec![];

    for input in monkey_input {
        println!("printing input {input}");
        let rows: Vec<&str> = input.lines().collect();

        println!("{rows:?}");

        let mut items: Vec<i64> = vec![];
        let mut monkey_parsing: Vec<&str> = rows[1].trim().split("Starting items: ").collect();
        monkey_parsing = monkey_parsing[1].split(", ").collect();
        for number in monkey_parsing {
            let item: i64 = number.parse::<i64>().unwrap();
            items.push(item);
        }

        monkey_parsing = rows[2].trim().split("Operation: new = old ").collect();
        monkey_parsing = monkey_parsing[1].split(" ").collect();
        let mut operation = monkey_parsing[0].chars().nth(0).unwrap();
        let operation_value: i64;
        if monkey_parsing[1] == "old" {
            operation = '^';
            operation_value = 2;
        } else {
            operation_value = monkey_parsing[1].parse::<i64>().unwrap();
        }

        monkey_parsing = rows[3].trim().split("Test: divisible by ").collect();
        let divisible_test: i64 = monkey_parsing[1].parse::<i64>().unwrap();

        monkey_parsing = rows[4].trim().split("If true: throw to monkey ").collect();
        let throw_to_monkey_if_true = monkey_parsing[1].parse::<usize>().unwrap();
        monkey_parsing = rows[5].trim().split("If false: throw to monkey ").collect();
        let throw_to_monkey_if_false = monkey_parsing[1].parse::<usize>().unwrap();

        let monkey = Monkey {
            items: RefCell::new(items),
            operation: operation,
            operation_value: operation_value,
            divisible_test: divisible_test,
            throw_to_monkey_if_true: throw_to_monkey_if_true,
            throw_to_monkey_if_false: throw_to_monkey_if_false,
            item_inspection_count: RefCell::new(0),
        };
        monkeys.push(monkey);
    }

    return monkeys;
}

fn monkey_biz(rounds: i32, monkeys: &Vec<Monkey>, part2: bool) {
    for _ in 0..rounds {
        for monkey in monkeys {
            let mut items = monkey.items.borrow_mut();
            items.reverse();
            while items.len() > 0 {
                // monkey inspects *does operation*
                let mut item = items.pop().unwrap();
                let operation = &monkey.operation;
                let value = &monkey.operation_value;
                match operation {
                    '+' => item += value,
                    '*' => item *= value,
                    '^' => item *= item,
                    _ => {}
                }

                // item / 3 round down nearest int
                if !part2 {
                    item /= 3;
                }

                // test divisible & throw item
                if item % monkey.divisible_test == 0 {
                    monkeys[monkey.throw_to_monkey_if_true]
                        .items
                        .borrow_mut()
                        .push(item);
                } else {
                    monkeys[monkey.throw_to_monkey_if_false]
                        .items
                        .borrow_mut()
                        .push(item);
                }
                *monkey.item_inspection_count.borrow_mut() += 1;
            }
        }
    }
}

fn get_monkey_biz_level(monkeys: &Vec<Monkey>) -> i64 {
    let mut monkey_inspection_counts: Vec<i64> = vec![];

    for monkey in monkeys {
        let count: i64 = *monkey.item_inspection_count.borrow() as i64;
        println!("monkeys count is {count:?}");
        monkey_inspection_counts.push(count);
    }
    monkey_inspection_counts.sort();
    monkey_inspection_counts.reverse();

    println!(" {monkey_inspection_counts:?}");

    return monkey_inspection_counts[0] * monkey_inspection_counts[1];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");

        let monkey_vec = get_monkey_vec(input);
        assert_eq!(part_1(&monkey_vec), 10605);
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let monkey_vec = get_monkey_vec(input);
        assert_eq!(part_2(&monkey_vec), 0);
    }
}
