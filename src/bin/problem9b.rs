use std::fs;
use std::collections::HashSet;

use itertools::Itertools;

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

fn main() {
    let file_path = "inputs/problem9.in";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines().collect_vec();

    let mut positions = vec![(0, 0); 10];

    let mut seen = HashSet::new();
    seen.insert((0, 0));

    for line in lines {
        let number_ops = &line[2..].parse::<i32>().unwrap();

        let first_move = match line.chars().nth(0) {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            Some('U') => Direction::Up,
            Some('D') => Direction::Down,
            Some(_) => panic!("Found invalid start of line!"),
            None => {println!("Found empty line! Skipping..."); continue}
        };

        for _ in 0..*number_ops {
            apply_direction(first_move, &mut positions);
            seen.insert(*positions.last().unwrap());
        }
    }

    println!("{:?}", seen.len());
}

fn apply_direction(first_move: Direction, positions: &mut Vec<(i32, i32)>) {
    let mut curr_move = first_move;

    for i in 0..positions.len() {
        let result = match curr_move {
            Direction::Up => move_u(positions.get(i).unwrap(), positions.get(i+1)),
            Direction::UpRight => move_ur(positions.get(i).unwrap(), positions.get(i+1)),
            Direction::Right => move_r(positions.get(i).unwrap(), positions.get(i+1)),
            Direction::DownRight => move_dr(positions.get(i).unwrap(), positions.get(i+1)),
            Direction::Down => move_d(positions.get(i).unwrap(), positions.get(i+1)),
            Direction::DownLeft => move_dl(positions.get(i).unwrap(), positions.get(i+1)),
            Direction::Left => move_l(positions.get(i).unwrap(), positions.get(i+1)),
            Direction::UpLeft => move_ul(positions.get(i).unwrap(), positions.get(i+1)),
        };

        if result.1.is_none() {
            break;
        }

        positions[i] = result.0;
        curr_move = result.1;
    }
}

fn move_l(h_pos: &(i32, i32), t_pos: Option<&(i32, i32)>) -> ((i32, i32), Option<Direction>) {
    let result_pos = h_pos.clone();
    result_pos.0 -= 1;

    if t_pos.is_none() {
        return (result_pos, None);
    }

    let next_pos = t_pos.unwrap().clone();
    
    if (result_pos.0 - next_pos.0).abs() > 1 {
        if result_pos.1 == next_pos.1 {
            (result_pos, Some(Direction::Left));
        } else if result_pos.1 < next_pos.1 {
            (result_pos, Some(Direction::DownLeft));
        } else {
            (result_pos, Some(Direction::UpLeft));
        }
    }

    (result_pos, None)
}

fn move_r(h_pos: &(i32, i32), t_pos: Option<&(i32, i32)>) -> ((i32, i32), Option<Direction>) {
    let result_pos = h_pos.clone();
    result_pos.0 += 1;

    if t_pos.is_none() {
        return (result_pos, None);
    }

    let next_pos = t_pos.unwrap().clone();
    
    if (result_pos.0 - next_pos.0).abs() > 1 {
        if result_pos.1 == next_pos.1 {
            (result_pos, Some(Direction::Right));
        } else if result_pos.1 < next_pos.1 {
            (result_pos, Some(Direction::DownRight));
        } else {
            (result_pos, Some(Direction::UpRight));
        }
    }

    (result_pos, None)
}

fn move_u(h_pos: &(i32, i32), t_pos: Option<&(i32, i32)>) -> ((i32, i32), Option<Direction>) {
    let result_pos = h_pos.clone();
    result_pos.1 -= 1;

    if t_pos.is_none() {
        return (result_pos, None);
    }

    let next_pos = t_pos.unwrap().clone();
    
    if (result_pos.1 - next_pos.1).abs() > 1 {
        if result_pos.0 == next_pos.0 {
            (result_pos, Some(Direction::Up));
        } else if result_pos.0 < next_pos.0 {
            (result_pos, Some(Direction::UpLeft));
        } else {
            (result_pos, Some(Direction::UpRight));
        }
    }

    (result_pos, None)
}

fn move_d(h_pos: &(i32, i32), t_pos: Option<&(i32, i32)>) -> ((i32, i32), Option<Direction>) {
    let result_pos = h_pos.clone();
    result_pos.1 += 1;

    if t_pos.is_none() {
        return (result_pos, None);
    }

    let next_pos = t_pos.unwrap().clone();
    
    if (result_pos.1 - next_pos.1).abs() > 1 {
        if result_pos.0 == next_pos.0 {
            (result_pos, Some(Direction::Down));
        } else if result_pos.0 < next_pos.0 {
            (result_pos, Some(Direction::DownLeft));
        } else {
            (result_pos, Some(Direction::DownRight));
        }
    }

    (result_pos, None)
}

fn move_ur(h_pos: &(i32, i32), t_pos: Option<&(i32, i32)>) -> ((i32, i32), Option<Direction>) {
    let result_pos = h_pos.clone();
    result_pos.0 += 1;
    result_pos.1 -= 1;

    if t_pos.is_none() {
        return (result_pos, None);
    }

    let next_pos = t_pos.unwrap().clone();

}

fn move_res(dir: Direction, h_pos: &(i32, i32), t_pos: Option<&(i32, i32)>) -> ((i32, i32), Option<Direction>) {
    let new_pos = h_pos.clone();
    
    match dir {
        Direction::Up => new_pos.1 -= 1,
        Direction::UpRight => {new_pos.0 += 1; new_pos.1 -= 1},
        Direction::Right => new_pos += (0, -1),
        Direction::DownRight => new_pos += (0, -1),
        Direction::Down => new_pos += (0, -1),
        Direction::DownLeft => new_pos += (0, -1),
        Direction::Left => new_pos += (0, -1),
        Direction::UpLeft => new_pos += (0, -1),
    };
}
