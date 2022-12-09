use std::fs;

use itertools::Itertools;

fn main() {
    let file_path = "inputs/problem8.in";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines().collect_vec();
    let mut seen = vec![vec![false; lines[0].len()]; lines.len()];

    // Scan in all 4 directions
    update_seen_horizontal(&lines, &mut seen);
    update_seen_horizontal_rev(&lines, &mut seen);
    update_seen_vertical(&lines, &mut seen);
    update_seen_vertical_rev(&lines, &mut seen);
    
    let result: u32 = seen.iter().map(|line| line.iter().filter(|x| **x).count() as u32).sum();

    println!("{result}");
}

fn update_seen_horizontal(lines: &Vec<&str>, seen: &mut Vec<Vec<bool>>) {
    for i in 0..lines.len() {
        let mut min_seen = -1;
        for j in 0..lines[0].len() {
            let current = lines[i].chars().nth(j).unwrap().to_digit(10).unwrap() as i32;
            if current > min_seen {
                seen[i][j] = true;
                min_seen = current;
            }
        }
    }
}

fn update_seen_horizontal_rev(lines: &Vec<&str>, seen: &mut Vec<Vec<bool>>) {
    for i in 0..lines.len() {
        let mut min_seen = -1;
        for j in (0..lines[0].len()).rev() {
            let current = lines[i].chars().nth(j).unwrap().to_digit(10).unwrap() as i32;
            if current > min_seen {
                seen[i][j] = true;
                min_seen = current;
            }
        }
    }
}

fn update_seen_vertical(lines: &Vec<&str>, seen: &mut Vec<Vec<bool>>) {
    for j in 0..lines.len() {
        let mut min_seen = -1;
        for i in 0..lines[0].len() {
            let current = lines[i].chars().nth(j).unwrap().to_digit(10).unwrap() as i32;
            if current > min_seen {
                seen[i][j] = true;
                min_seen = current;
            }
        }
    }
}

fn update_seen_vertical_rev(lines: &Vec<&str>, seen: &mut Vec<Vec<bool>>) {
    for j in 0..lines.len() {
        let mut min_seen = -1;
        for i in (0..lines[0].len()).rev() {
            let current = lines[i].chars().nth(j).unwrap().to_digit(10).unwrap() as i32;
            if current > min_seen {
                seen[i][j] = true;
                min_seen = current;
            }
        }
    }
}