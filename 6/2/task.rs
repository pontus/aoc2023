use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn main() {
    let mut reader = io::BufReader::new(File::open("input.txt").unwrap());
    let mut lines = (&mut reader).lines();

    let timeline = lines.next().unwrap().unwrap();
    let timestring = timeline.split(":").collect::<Vec<&str>>()[1].replace(" ", "");
    let scoreline = lines.next().unwrap().unwrap();
    let scorestring = scoreline.split(":").collect::<Vec<&str>>()[1].replace(" ", "");

    let mut times: Vec<u64> = Vec::new();
    let mut scores: Vec<u64> = Vec::new();

    println!("Seeing time strings {:?}", timestring);

    times.push(timestring.parse::<u64>().unwrap());

    scores.push(scorestring.parse::<u64>().unwrap());

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
