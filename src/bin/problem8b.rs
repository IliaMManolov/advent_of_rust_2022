use std::fs;

use itertools::Itertools;

fn main() {
    let file_path = "inputs/problem8.in";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines().collect_vec();
    let mut scores = vec![vec![(0, 0, 0, 0); lines[0].len()]; lines.len()];

    // Scan in all 4 directions
    update_scores_horizontal(&lines, &mut scores);
    update_scores_horizontal_rev(&lines, &mut scores);
    update_scores_vertical(&lines, &mut scores);
    update_scores_vertical_rev(&lines, &mut scores);

    let result: u32 = scores
        .iter()
        .map(|line| line.iter().map(|x| x.0 * x.1 * x.2 * x.3).max().unwrap())
        .max()
        .unwrap();

    println!("{result}");
}

fn update_scores_horizontal(lines: &Vec<&str>, scores: &mut Vec<Vec<(u32, u32, u32, u32)>>) {
    for i in 0..lines.len() {
        let mut visible_heights_pos = Vec::new();
        for j in 0..lines[0].len() {
            let current = lines[i].chars().nth(j).unwrap().to_digit(10).unwrap() as i32;
            if visible_heights_pos.len() == 0 {
                scores[i][j].0 = 0;
                visible_heights_pos.push((current, j));
            } else {
                let first_smaller_opt = visible_heights_pos.iter().position(|x| x.0 < current);
                if first_smaller_opt.is_none() {
                    scores[i][j].0 = 1;
                    visible_heights_pos.push((current, j));
                } else {
                    let first_smaller = first_smaller_opt.unwrap();
                    for _ in first_smaller..visible_heights_pos.len() {
                        visible_heights_pos.pop();
                    }
                    scores[i][j].0 = j as u32 - visible_heights_pos.last().unwrap_or(&(0, 0)).1 as u32;
                    visible_heights_pos.push((current, j));
                }
            }
        }
    }
}

fn update_scores_horizontal_rev(lines: &Vec<&str>, scores: &mut Vec<Vec<(u32, u32, u32, u32)>>) {
    for i in 0..lines.len() {
        let mut visible_heights_pos = Vec::new();
        for j in (0..lines[0].len()).rev() {
            let current = lines[i].chars().nth(j).unwrap().to_digit(10).unwrap() as i32;
            if visible_heights_pos.len() == 0 {
                scores[i][j].1 = 0;
                visible_heights_pos.push((current, j));
            } else {
                let first_smaller_opt = visible_heights_pos.iter().position(|x| x.0 < current);
                if first_smaller_opt.is_none() {
                    scores[i][j].1 = 1;
                    visible_heights_pos.push((current, j));
                } else {
                    let first_smaller = first_smaller_opt.unwrap();
                    for _ in first_smaller..visible_heights_pos.len() {
                        visible_heights_pos.pop();
                    }
                    scores[i][j].1 = visible_heights_pos.last().unwrap_or(&(0, lines[0].len() - 1)).1 as u32 - j as u32;
                    visible_heights_pos.push((current, j));
                }
            }
        }
    }
}

fn update_scores_vertical(lines: &Vec<&str>, scores: &mut Vec<Vec<(u32, u32, u32, u32)>>) {
    for j in 0..lines[0].len() {
        let mut visible_heights_pos = Vec::new();
        for i in 0..lines.len() {
            let current = lines[i].chars().nth(j).unwrap().to_digit(10).unwrap() as i32;
            if visible_heights_pos.len() == 0 {
                scores[i][j].2 = 0;
                visible_heights_pos.push((current, i));
            } else {
                let first_smaller_opt = visible_heights_pos.iter().position(|x| x.0 < current);
                if first_smaller_opt.is_none() {
                    scores[i][j].2 = 1;
                    visible_heights_pos.push((current, i));
                } else {
                    let first_smaller = first_smaller_opt.unwrap();
                    for _ in first_smaller..visible_heights_pos.len() {
                        visible_heights_pos.pop();
                    }
                    scores[i][j].2 = i as u32 - visible_heights_pos.last().unwrap_or(&(0, 0)).1 as u32;
                    visible_heights_pos.push((current, i));
                }
            }
        }
    }
}

fn update_scores_vertical_rev(lines: &Vec<&str>, scores: &mut Vec<Vec<(u32, u32, u32, u32)>>) {
    for j in 0..lines[0].len() {
        let mut visible_heights_pos = Vec::new();
        for i in (0..lines.len()).rev() {
            let current = lines[i].chars().nth(j).unwrap().to_digit(10).unwrap() as i32;
            if visible_heights_pos.len() == 0 {
                scores[i][j].3 = 0;
                visible_heights_pos.push((current, i));
            } else {
                let first_smaller_opt = visible_heights_pos.iter().position(|x| x.0 < current);
                if first_smaller_opt.is_none() {
                    scores[i][j].3 = 1;
                    visible_heights_pos.push((current, i));
                } else {
                    let first_smaller = first_smaller_opt.unwrap();
                    for _ in first_smaller..visible_heights_pos.len() {
                        visible_heights_pos.pop();
                    }
                    scores[i][j].3 = visible_heights_pos.last().unwrap_or(&(0, lines.len() - 1)).1 as u32 - i as u32;
                    visible_heights_pos.push((current, i));
                }
            }
        }
    }
}
