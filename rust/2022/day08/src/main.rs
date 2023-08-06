use ::std::collections::HashSet;

fn main() {
    let input_path = format!("../../../input/2022/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let row: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut trees: Vec<Vec<i32>> = vec![];
    for r in row {
        let mut temp_row: Vec<i32> = vec![];
        for c in r.chars() {
            temp_row.push(c as i32 - 48);
        }
        trees.push(temp_row)
    }

    println!("part 1 {}\npart 2 {}", part_1(&trees), part_2(&trees))
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

fn part_2(trees: &Vec<Vec<i32>>) -> i32 {
    let row_len = trees[0].len();
    let col_len = trees.len();

    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut scenic_score: Vec<i32> = vec![];
    for i in 0..row_len {
        for j in 0..col_len {
            let mut tree_count: Vec<i32> = vec![];

            for d in directions {
                let mut count = 0;
                let mut di: i32 = i.try_into().unwrap();
                let mut dj: i32 = j.try_into().unwrap();
                loop {
                    count += 1;
                    di += d.0;
                    dj += d.1;

                    if !(0 <= di
                        && di < row_len.try_into().unwrap()
                        && 0 <= dj
                        && dj < col_len.try_into().unwrap())
                    {
                        // if out of bounds, break and remove 1 count
                        count -= 1;
                        break;
                    } else if trees[di as usize][dj as usize] >= trees[i][j] {
                        // if a tree is taller than i j
                        break;
                    }
                }
                tree_count.push(count);
            }
            let total = tree_count[0] * tree_count[1] * tree_count[2] * tree_count[3];
            scenic_score.push(total);
        }
    }
    scenic_score.sort();
    return scenic_score[scenic_score.len() - 1];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_path = format!("../../../input/2022/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
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
        let input_path = format!("../../../input/2022/{}_test.txt", env!("CARGO_PKG_NAME"));
        let input: String = std::fs::read_to_string(input_path).expect("can't read file");
        let row: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
        let mut trees: Vec<Vec<i32>> = vec![];
        for r in row {
            let mut temp_row: Vec<i32> = vec![];
            for c in r.chars() {
                temp_row.push(c as i32 - 8);
            }
            trees.push(temp_row)
        }

        assert_eq!(part_2(&trees), 99);
    }
}
