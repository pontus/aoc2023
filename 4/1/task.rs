use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let max: HashMap<&str, u64> =
        HashMap::<&str, u64>::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut sum = 0;

    for l in lines {
        let s = l.unwrap();
        let v: Vec<&str> = s.split(":").collect();
        let s: Vec<&str> = v[1].split("|").collect();
        //let mut cards: Vec<(usize)> = Vec::new();

        let mut score = 0;
        let winning: Vec<&str> = s[0].split(" ").collect();
        let onhand: Vec<&str> = s[1].split(" ").collect();
        //let game = gamebit[1].parse::<u64>().unwrap();

        for p in winning.into_iter() {
            if p.eq("") {
                continue;
            }

            if onhand.contains(&p) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
            println!("*{p}* {score}")
        }

        sum += score;
    }

    println!("{:?}", sum);
}
