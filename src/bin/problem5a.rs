use std::{fs, vec};

use itertools::Itertools;

fn main() {
    let file_path = "inputs/problem5.in";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines().collect_vec();

    // Split input into state and moves
    let input1 = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .collect_vec();
    let input2 = lines
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect_vec();

    let mut current_state = create_state(input1);
    let moves = create_moves(input2);

    for action in moves {
        for _ in 0..action.0 {
            let moved_elem = current_state[action.1 as usize].pop().unwrap();
            current_state[action.2 as usize].push(moved_elem);
        }
    }

    println!(
        "{}",
        current_state
            .iter()
            .map(|x| x.last().unwrap())
            .collect::<String>()
    );
}

fn create_state(lines: Vec<&&str>) -> Vec<Vec<char>> {
    // Turn the strings to lists of container chars
    let pretransposed = lines
        .iter()
        .map(|line| {
            line.chars()
                .chunks(4)
                .into_iter()
                .map(|chunk| {
                    let chunk_cpy = chunk.collect_vec();
                    if chunk_cpy.iter().all(|x| x.is_whitespace())
                        || chunk_cpy.iter().any(|x| x.is_numeric())
                    {
                        None
                    } else {
                        Some(chunk_cpy[1])
                    }
                })
                .collect::<Vec<Option<char>>>()
        })
        .collect_vec();

    // Transpose matrix
    let mut entries = vec![Vec::<char>::new(); pretransposed[0].len()];
    for row in pretransposed {
        for (i, col) in row.iter().enumerate() {
            if col.is_some() {
                entries[i].push(col.unwrap());
            }
        }
    }

    // Reverse every row since backmost element should be topmost of input
    for row in entries.iter_mut() {
        row.reverse();
    }

    entries
}

fn create_moves(lines: Vec<&&str>) -> Vec<(u32, u32, u32)> {
    lines
        .iter()
        .map(|line| {
            let line_split = line.split_whitespace().collect_vec();
            (
                line_split[1].parse::<u32>().unwrap(),
                line_split[3].parse::<u32>().unwrap() - 1,
                line_split[5].parse::<u32>().unwrap() - 1,
            )
        })
        .collect_vec()
}
