use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("inputs/problem10.in") {
        let mut reg_val: i32 = 1;
        let mut cycle_cnt: i32 = 0;
        let mut total: i32 = 0;

        for line in lines {
            if let Ok(ip) = line {
                match ip.split_whitespace().nth(0).unwrap() {
                    "addx" => {
                        cycle_cnt += 1;
                        if (cycle_cnt - 20) % 40 == 0 {
                            total += cycle_cnt * reg_val;
                        }
                        cycle_cnt += 1;
                        if (cycle_cnt - 20) % 40 == 0 {
                            total += cycle_cnt * reg_val;
                        }
                        reg_val += ip
                            .split_whitespace()
                            .nth(1)
                            .unwrap()
                            .parse::<i32>()
                            .unwrap();
                    }
                    "noop" => {
                        cycle_cnt += 1;
                        if (cycle_cnt - 20) % 40 == 0 {
                            total += cycle_cnt * reg_val;
                        }
                    }
                    _ => panic!("Invalid command in input file"),
                };
            } else {
                println!("Oh no");
            }
        }

        println!("Total: {total}");
    } else {
        println!("Failed to open file!")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
