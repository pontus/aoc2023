use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut sum = 0;

    let mut extras: Vec<usize> = Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    for l in lines {
        let s = l.unwrap();
        let v: Vec<&str> = s.split(":").collect();
        let s: Vec<&str> = v[1].split("|").collect();
        //let mut cards: Vec<(usize)> = Vec::new();

        let winning: Vec<&str> = s[0].split(" ").collect();
        let onhand: Vec<&str> = s[1].split(" ").collect();
        //let game = gamebit[1].parse::<u64>().unwrap();

        let extrarun = extras.remove(0);
        extras.push(0);

        let mut i = 0;

        println!("{:?}, processing extra {extrarun}", v[0]);

        while i < 1 + extrarun {
            let mut copies = 0;
            println!("New processing");

            for p in winning.clone().into_iter() {
                if p.eq("") {
                    continue;
                }

                if onhand.contains(&p) {
                    copies += 1;
                }
                println!("*{p}* {copies}")
            }

            let mut j = 0;
            while j < copies {
                println!("j = {j} copies={copies}");

                extras[j] += 1;
                j += 1;
            }

            sum += 1; // Number of processed cards
            i += 1;
        }
    }

    println!("{:?}", sum);
}
