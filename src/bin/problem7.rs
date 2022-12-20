use std::fs;

#[derive(Debug)]
struct FSNode {
    name: String,
    children: Option<Vec<usize>>, // Determines whether it's a directory
    parent: usize,
}

fn create_dir(tree: &mut Vec<FSNode>, sizes: &mut Vec<u32>, name: String, parent: usize) {
    let new_node = FSNode {
        name,
        children: Some(Vec::new()),
        parent,
    };

    let tree_len = tree.len();
    tree.push(new_node);

    if let Some(ref mut children_ref) = tree[parent].children {
        children_ref.push(tree_len);
        sizes.push(0);
    } else {
        panic!("You're trying to add a child to a folder, you dingus!")
    }
}

fn create_file(
    tree: &mut Vec<FSNode>,
    sizes: &mut Vec<u32>,
    name: String,
    size: u32,
    parent: usize,
) {
    let new_node = FSNode {
        name,
        children: None,
        parent,
    };

    let tree_len = tree.len();
    tree.push(new_node);

    if let Some(ref mut children_ref) = tree[parent].children {
        children_ref.push(tree_len);
        sizes.push(size);
    } else {
        panic!("You're trying to add a child to a folder, you dingus!")
    }
}

fn update_sizes(tree: &Vec<FSNode>, sizes: &mut Vec<u32>, target: usize) {
    if let Some(ref children) = tree[target].children {
        let mut target_size = 0;
        for child in children {
            update_sizes(tree, sizes, *child);
            target_size += sizes
                .get(*child)
                .expect("Tried to access outside of bounds on tree");
        }
        sizes[target] = target_size;
    }
    // Otherwise reached a file and no need to update
}

fn main() {
    let file_path = "inputs/problem7.in";
    let contents = fs::read_to_string(file_path).expect("Should be able to read file");
    let lines = contents.lines();

    let mut tree = Vec::<FSNode>::new();
    let mut sizes = Vec::<u32>::new();
    let mut curr_node = 0;

    tree.push(FSNode {
        name: "/".to_string(),
        children: Some(Vec::new()),
        parent: 0,
    });
    sizes.push(0);

    for line in lines {        
        if line.starts_with("$ ") {
            let mut words = line.split_whitespace();
            match words.nth(1).unwrap() {
                "cd" => {
                    if let Some(dest) = words.nth(0) {
                        if dest == "/" {
                            curr_node = 0;
                        } else if dest == ".." {
                            curr_node = tree[curr_node].parent;
                        } else {
                            for child_id in tree[curr_node].children.as_ref().unwrap() {
                                if tree[*child_id].name == dest {
                                    curr_node = *child_id;
                                }
                            }
                        }
                    }
                }
                "ls" => continue,
                _ => panic!("Invalid command found!"),
            }
        } else if line.starts_with("dir") {
            create_dir(
                &mut tree,
                &mut sizes,
                line.split_whitespace().nth(1).unwrap().to_string(),
                curr_node,
            );
        } else {
            let mut words = line.split_whitespace();
            let size = words.nth(0).unwrap().parse::<u32>().unwrap();
            let name = words.nth(0).unwrap().to_string();

            create_file(&mut tree, &mut sizes, name, size, curr_node);
        }
    }

    update_sizes(&tree, &mut sizes, 0);

    let mut result_a = 0;
    for (id, node) in tree.iter().enumerate() {
        if node.children.is_some() && sizes[id] <= 100000 {
            result_a += sizes[id];
        }
    }

    println!("Result of problem A: {result_a}");

    let mut result_b = sizes[0];
    for (id, node) in tree.iter().enumerate() {
        if node.children.is_some() && 40000000 + sizes[id] >= sizes[0] && sizes[id] < result_b {
            result_b = sizes[id];
        }
    }

    println!("Result of problem B: {result_b}");
}
