use std::{collections::HashMap};

fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let games: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();

    let scoring_table: HashMap<&str, i32> = HashMap::from([
        ("win", 6),
        ("lose", 0),
        ("draw", 3),
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
        ("A", 1),
        ("B", 2),
        ("C", 3),
    ]);

    println!(
        "part 1 {}\npart 2 {}",
        part_1(&games, &scoring_table),
        part_2(&games, &scoring_table)
    )
}

fn part_1(games: &Vec<&str>, scoring_table: &HashMap<&str, i32>) -> i32 {
    let mut score: i32 = 0;

    for game in games {
        let (opp, player) = game.split_once(' ').unwrap();

        // R P S / A B C / X Y Z
        if opp == "A" && player == "Y" || opp == "B" && player == "Z" || opp == "C" && player == "X"
        {
            score += *scoring_table.get(player).unwrap() + *scoring_table.get("win").unwrap();
        }
        if opp == "B" && player == "X" || opp == "C" && player == "Y" || opp == "A" && player == "Z"
        {
            score += *scoring_table.get(player).unwrap() + *scoring_table.get("lose").unwrap();
        }
        if opp == "A" && player == "X" || opp == "B" && player == "Y" || opp == "C" && player == "Z"
        {
            score += *scoring_table.get(player).unwrap() + *scoring_table.get("draw").unwrap();
        }
    }
    return score;
}

fn part_2(games: &Vec<&str>, scoring_table: &HashMap<&str, i32>) -> i32 {
    let mut score: i32 = 0;

    for game in games {
        let (opp, outcome) = game.split_once(' ').unwrap();
        // Outcomes: X lose, Y draw, Z win
        // R P S / A B C
        if opp == "A" && outcome == "X" {
            score += *scoring_table.get("C").unwrap() + *scoring_table.get("lose").unwrap();
        }
        if opp == "B" && outcome == "X" {
            score += *scoring_table.get("A").unwrap() + *scoring_table.get("lose").unwrap();
        }
        if opp == "C" && outcome == "X" {
            score += *scoring_table.get("B").unwrap() + *scoring_table.get("lose").unwrap();
        }

        //draw
        if outcome == "Y" {
            score += *scoring_table.get(opp).unwrap() + *scoring_table.get("draw").unwrap();
        }
        // lose
        if opp == "A" && outcome == "Z" {
            score += *scoring_table.get("B").unwrap() + *scoring_table.get("win").unwrap();
        }
        if opp == "B" && outcome == "Z" {
            score += *scoring_table.get("C").unwrap() + *scoring_table.get("win").unwrap();
        }
        if opp == "C" && outcome == "Z" {
            score += *scoring_table.get("A").unwrap() + *scoring_table.get("win").unwrap();
        }
    }
    return score;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let games: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();

        let scoring_table: HashMap<&str, i32> = HashMap::from([
            ("win", 6),
            ("lose", 0),
            ("draw", 3),
            ("X", 1),
            ("Y", 2),
            ("Z", 3),
            ("A", 1),
            ("B", 2),
            ("C", 3),
        ]);

        assert_eq!(part_1(&games, &scoring_table), 15);
    }
    #[test]
    fn test_part_2() {
        let input: String =
            std::fs::read_to_string("../../input/day2_test.txt").expect("can't read file");
        let games: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();

        let scoring_table: HashMap<&str, i32> = HashMap::from([
            ("win", 6),
            ("lose", 0),
            ("draw", 3),
            ("X", 1),
            ("Y", 2),
            ("Z", 3),
            ("A", 1),
            ("B", 2),
            ("C", 3),
        ]);
        assert_eq!(part_2(&games, &scoring_table), 12);
    }
}
