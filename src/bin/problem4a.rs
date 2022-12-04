use itertools::Itertools;
use std::fs;

fn main() {
    let file_path = "inputs/problem4.in";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut result = 0;

    for line in lines {
        let range_one: (i32, i32) = line
            .split(|c| c == ',' || c == '-')
            .take(2)
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let range_two: (i32, i32) = line
            .split(|c| c == ',' || c == '-')
            .skip(2)
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();

        result += ((range_one.0 <= range_two.0 && range_one.1 >= range_two.1)
            || (range_one.0 >= range_two.0 && range_one.1 <= range_two.1)) as i32;
    }

    println!("There are {result} elves with fully overlapping assignments.");
}
