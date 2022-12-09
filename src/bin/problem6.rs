use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::prelude::*;

// Uncomment top line for problem A and bottom one for problem B
// const number_of_distinct: u32 = 4;
const NUMBER_OF_DISTINCT: u32 = 14;

fn main() -> io::Result<()> {


    let mut f = File::open("inputs/problem6.in")?;

    // Use this to read 1 byte at a time
    let mut buffer = [0; 1];

    let mut seen_queue = VecDeque::new();

    for _ in 0..NUMBER_OF_DISTINCT {
        f.read(&mut buffer)?;
        seen_queue.push_back(buffer[0]);
    }

    let mut result = NUMBER_OF_DISTINCT;

    while !check_done(&seen_queue) {
        f.read(&mut buffer)?;
        seen_queue.push_back(buffer[0]);
        seen_queue.pop_front();
        result += 1;
    }

    println!("{result}");

    Ok(())
}

fn check_done(seen_queue: &VecDeque<u8>) -> bool {
    // Check this by dumping into a set which has no duplicates
    seen_queue.into_iter().collect::<HashSet<_>>().len() == NUMBER_OF_DISTINCT as usize
}
