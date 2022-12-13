use ::std::collections::HashSet;

fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
    let mut trees: Vec<Vec<i32>> = vec![];
    for r in row {
        let mut temp_row: Vec<i32> = vec![];
        for c in r.chars() {
            temp_row.push(c as i32 - 48);
        }
        trees.push(temp_row)
    }

    println!("part 1 {}\npart 2 {}", part_1(&trees), part_2())
}

fn part_1(trees: &Vec<Vec<i32>>) -> usize {
    let mut trees_visible = HashSet::new();
    let row_len = trees[0].len();
    let col_len = trees.len();

    for j in 0..col_len {
        let mut tallest_on_row: i32 = -1;
        let mut tallest_on_rev_row: i32 = -1;
        for i in 0..row_len {
            if tallest_on_row < trees[i][j] {
                tallest_on_row = trees[i][j];
                trees_visible.insert((i, j));
            }
        }

        for i in 0..row_len {
            let rev_iterator: usize = (row_len as i32 - i as i32 - 1).try_into().unwrap();
            if tallest_on_rev_row < trees[rev_iterator][j] {
                tallest_on_rev_row = trees[rev_iterator][j];
                trees_visible.insert((rev_iterator, j));
            }
        }
    }

    for j in 0..row_len {
        let mut tallest_on_col: i32 = -1;
        let mut tallest_on_rev_col: i32 = -1;
        for i in 0..col_len {
            if tallest_on_col < trees[j][i] {
                tallest_on_col = trees[j][i];
                trees_visible.insert((j, i));
            }
        }
        for i in 0..col_len {
            let rev_iterator: usize = (col_len as i32 - i as i32 - 1).try_into().unwrap();
            if tallest_on_rev_col < trees[j][rev_iterator] {
                tallest_on_rev_col = trees[j][rev_iterator];
                trees_visible.insert((j, rev_iterator));
            }
        }
    }
    return trees_visible.len();
}

fn part_2() -> i32 {
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
        let mut trees: Vec<Vec<i32>> = vec![];
        for r in row {
            let mut temp_row: Vec<i32> = vec![];
            for c in r.chars() {
                temp_row.push(c as i32 - 48);
            }
            trees.push(temp_row)
        }

        assert_eq!(part_1(&trees), 21);
    }
    #[test]
    fn test_part_2() {
        let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
        assert_eq!(part_2(), 0);
    }
}
