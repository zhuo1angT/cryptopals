use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let file_contents = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();

    let mut sets: Vec<HashSet<String>> = vec![HashSet::new(); lines.len()];

    for (index, line) in lines.iter().enumerate() {
        let mut j = 0;
        while (j + 1) * 16 <= lines.len() {
            sets[index].insert((&line[j * 16..(j + 1) * 16]).to_string());
            j += 1;
        }
    }

    let mut min_size = usize::MAX;
    let mut line_number = 0;
    for (index, set) in sets.iter().enumerate() {
        if set.len() < min_size {
            min_size = set.len();
            line_number = index;
        }
    }
    if min_size != usize::MAX {
        println!(
            "line number: {}, number of unique blocks: {}",
            line_number, min_size
        );
        println!("{}", lines[line_number]);
    }
}
