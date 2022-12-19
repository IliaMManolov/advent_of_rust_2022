use std::fs;
use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let file_path = "inputs/problem9.in";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines().collect_vec();

    let mut h_pos = (0, 0);
    let mut t_pos = (0, 0);

    let mut seen = HashSet::new();
    seen.insert(t_pos);

    for line in lines {
        let number_ops = &line[2..].parse::<i32>().unwrap();

        match line.chars().nth(0) {
            Some('L') => for _ in 0..*number_ops {move_l(&mut h_pos, &mut t_pos); seen.insert(t_pos);}
            Some('R') => for _ in 0..*number_ops {move_r(&mut h_pos, &mut t_pos); seen.insert(t_pos);}
            Some('U') => for _ in 0..*number_ops {move_u(&mut h_pos, &mut t_pos); seen.insert(t_pos);}
            Some('D') => for _ in 0..*number_ops {move_d(&mut h_pos, &mut t_pos); seen.insert(t_pos);}
            Some(_) => panic!("Found invalid start of line!"),
            None => {println!("Found empty line! Skipping..."); continue}
        }
    }

    println!("{:?}", seen.len());
}

fn move_l(h_pos: &mut (i32, i32), t_pos: &mut (i32, i32)) {
    h_pos.0 -= 1;
    if (h_pos.0 - t_pos.0).abs() > 1 {
        t_pos.0 = h_pos.0 + 1;
        t_pos.1 = h_pos.1;
    }
}

fn move_r(h_pos: &mut (i32, i32), t_pos: &mut (i32, i32)) {
    h_pos.0 += 1;
    if (h_pos.0 - t_pos.0).abs() > 1 {
        t_pos.0 = h_pos.0 - 1;
        t_pos.1 = h_pos.1;
    }
}

fn move_u(h_pos: &mut (i32, i32), t_pos: &mut (i32, i32)) {
    h_pos.1 -= 1;
    if (h_pos.1 - t_pos.1).abs() > 1 {
        t_pos.1 = h_pos.1 + 1;
        t_pos.0 = h_pos.0;
    }
}

fn move_d(h_pos: &mut (i32, i32), t_pos: &mut (i32, i32)) {
    h_pos.1 += 1;
    if (h_pos.1 - t_pos.1).abs() > 1 {
        t_pos.1 = h_pos.1 - 1;
        t_pos.0 = h_pos.0;
    }
}