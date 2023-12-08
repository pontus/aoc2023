use std::collections::HashMap;
use std::fs::File;
use std::io::read_to_string;
use std::vec::Vec;

fn main() {
    let reader = read_to_string(File::open("input.txt").unwrap()).unwrap();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::<&str, (&str, &str)>::new();

    let mut sum = 0;

    let mut hands: Vec<(&str, usize)> = Vec::new();
    let mut instructions = "";

    for (linenumber, l) in reader.lines().enumerate() {
        if linenumber == 0 {
            instructions = &l.trim();
            continue;
        }

        if l.eq("") {
            continue;
        }

        let key = &l[0..3];
        let left = &l[7..10];
        let right = &l[12..15];

        map.insert(key, (left, right));
        println!("{key} {left} {right}");
    }

    let mut at = "AAA";

    while at.ne("ZZZ") {
        let mut i: usize = 0;
        while i < instructions.len() {
            let c = &(&instructions)[i..i + 1];
            let frommap = map[at];
            if c.eq("L") {
                at = frommap.0;
            } else {
                at = frommap.1;
            }
            sum += 1;
            i += 1;
        }
    }

    println!("{:?}", sum);
}
