use num::integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::read_to_string;
use std::vec::Vec;

fn main() {
    let reader = read_to_string(File::open("input.txt").unwrap()).unwrap();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::<&str, (&str, &str)>::new();

    let mut sum = 0;

    let mut at: Vec<&str> = Vec::new();
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

        if key[2..3].eq("A") {
            at.push(key);
        }
    }

    println!("{:?}", at);

    let mut counts: Vec<u64> = Vec::new();
    let mut doat = 0;
    while doat < at.len() {
        let mut thissum = 0;
        let mut thisat = at[doat];
        let mut i: usize = 0;
        while thisat[2..3].ne("Z") {
            let c = &(&instructions)[i..i + 1];

            let frommap = map[thisat];
            if c.eq("L") {
                thisat = frommap.0;
            } else {
                thisat = frommap.1;
            }

            thissum += 1;
            i = (i + 1) % instructions.len();
        }
        counts.push(thissum);
        doat += 1;
    }
    let mut lc = 1;
    for p in counts.iter() {
        lc = lcm(lc, *p);
    }

    println!("{:?} {lc}", counts);
}
