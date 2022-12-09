use std::{io, fs, collections::HashMap};

use itertools::Itertools;

fn recurse(directory: &String, part_sizes: &HashMap<String, u32>, subdirs: &HashMap<String, Vec<String>>, sizes: &HashMap<String, u32>) {
    sizes[directory] = part_sizes[directory];
    for subdir in &subdirs[directory] {
        if !sizes.contains_key(subdir) {
            recurse(subdir, part_sizes, subdirs, sizes);
        }
        sizes[directory] += sizes[subdir];
    }
}

fn main() -> io::Result<()> {
    let file_path = "inputs/problem7.in";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines().collect_vec();
    
    let mut curr = "".to_string();

    let mut part_sizes: HashMap<String, u32> = HashMap::<String, u32>::new();
    let mut sizes: HashMap<String, u32> = HashMap::<String, u32>::new();
    let mut subdirs: HashMap<String, Vec<str>> = HashMap::<String, Vec<str>>::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line == "$ ls" {
            i += 1;
            while i < lines.len() && !lines[i].starts_with("$") {
                if lines[i].starts_with("dir") {
                    let subdir = lines[i].split(' ').skip(1).next().unwrap();
                    subdirs[&curr].push(subdir);
                }
                else {
                    let size = line.split(" ").next().unwrap().parse::<u32>().unwrap();
                    part_sizes[&curr] += size;
                }
                i += 1;
            }
            i -= 1;
        }
        else if line == "$ cd .." {
            let parts = curr.split("😳").collect_vec();
            curr = parts[0..parts.len()-1].join("😳");
        }
        else if line.starts_with("$ cd ") {
            let sub = &line.replace("$ cd ", "");
            curr += &("😳".to_string() + &sub.to_string());
            if !part_sizes.contains_key(&curr) {
                part_sizes[&curr] = 0;
            }
        }
        else {
            println!("BAD {:?}", i);
        }
        //println!("{:?} {:?}", line, curr);
        i += 1;
    }

    recurse(&"😳/".to_string(), &part_sizes, &subdirs, &sizes);

    println!("{:?}", sizes);
    
    Ok(())
}