fn main() {
    let input_path = format!("../../input/{}.txt", env!("CARGO_PKG_NAME"));
    let input: String = std::fs::read_to_string(input_path).expect("can't read file");
    let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();

    //println!("{:?}", input.len());
    println!("part 1 {}{}part 2 {}", part_1(&input), "\n", part_2(&row))
}

fn part_1(input: &String) -> i32 {
    let mut start_marker: Vec<String> = vec![];
    let input = input.clone();
    // for i in 0..3{

    // }
    let mut i = 0;
    while i < input.len() {
        println!("{:?}", i);
        println!("{:?}", input.len());
        let char_to_add = input.chars().nth(i).unwrap().to_string();
        start_marker.push(char_to_add);
        println!("startmarker len {:?}", start_marker.len());
        println!("start marker {:?}", start_marker);
        if i >= 4 {
            let window = vec![
                &start_marker[start_marker.len()],
                &start_marker[start_marker.len() - 1],
                &start_marker[start_marker.len() - 2],
            ];
            println!("window {:?}", window);
            for c in &window {
                println!("{}", c);
                if window.contains(c) {
                    println!("{:?}", "YAYAYA");
                }
            }
            start_marker.remove(0);
        }
        i += 1
    }
    return 0;
}

fn part_2(_row: &Vec<&str>) -> i32 {
    return 0;
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_part_1() {
//         let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
//         let input: String = std::fs::read_to_string(input_path).expect("can't read file");
//         let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
//         assert_eq!(part_1(&row), 5);
//     }
//     #[test]
//     fn test_part_2() {
//         let input_path = format!("../../input/{}_test.txt", env!("CARGO_PKG_NAME"));
//         let input: String = std::fs::read_to_string(input_path).expect("can't read file");
//         let row: Vec<&str> = input.split("\r\n").collect::<Vec<&str>>();
//         assert_eq!(part_2(&row), 0);
//     }
// }
