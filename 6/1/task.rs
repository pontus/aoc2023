use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn main() {
    let mut reader = io::BufReader::new(File::open("input.txt").unwrap());
    let mut lines = (&mut reader).lines();

    let timeline = lines.next().unwrap().unwrap();
    let timelineparts: Vec<&str> = timeline.split(":").collect();

    let timestrings: Vec<&str> = timelineparts[1].split(" ").collect();

    let scoreline = lines.next().unwrap().unwrap();
    let scorelineparts: Vec<&str> = scoreline.split(":").collect();
    let scorestrings: Vec<&str> = scorelineparts[1].split(" ").collect();

    let mut times: Vec<u64> = Vec::new();
    let mut scores: Vec<u64> = Vec::new();

    println!("Seeing time strings {:?}", timestrings);

    for p in timestrings.into_iter() {
        if p.eq("") {
            continue;
        }
        let t = p.parse::<u64>().unwrap();

        times.push(t);
    }

    for p in scorestrings.into_iter() {
        if p.eq("") {
            continue;
        }
        let t = p.parse::<u64>().unwrap();

        scores.push(t);
    }

    println!("Doing races");

    let mut count = 1;
    let mut i = 0;
    while i < times.len() {
        let mut winnings = 0;
        let currentbest = scores[i];
        let heatlen = times[i];
        let mut j = 0;
        while j <= heatlen {
            if currentbest < j * (heatlen - j) {
                winnings += 1;
            }
            j += 1;
        }

        i += 1;
        println!("{winnings} ways to win");

        count *= winnings;
    }
    println!("{count}");
}
