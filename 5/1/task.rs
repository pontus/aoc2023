use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn main() {
    let mut reader = io::BufReader::new(File::open("input.txt").unwrap());
    let mut lines = (&mut reader).lines();

    let seedline = lines.next().unwrap().unwrap();
    let seedlineparts: Vec<&str> = seedline.split(":").collect();

    let seedsstrings: Vec<&str> = seedlineparts[1].split(" ").collect();
    let mut mappings: Vec<Vec<u64>> = Vec::new();

    for p in 0..10 {
        mappings.push(Vec::new());
    }

    println!("Seeing seed strings {:?}", seedlineparts);

    for p in seedsstrings.into_iter() {
        if p.eq("") {
            continue;
        }
        let seed = p.parse::<u64>().unwrap();
        println!("Seeing seed {seed}");

        mappings[0].push(seed);
    }

    lines.next();
    lines.next();

    for srcmapno in 0..7 {
        println!("Start; Current generation {:?}", mappings[srcmapno]);

        for l in (&mut reader).lines() {
            let s = l.unwrap();

            if s.trim().eq("") {
                // Advance to skip header
                (&mut reader).lines().next();
                break;
            }

            let nums: Vec<&str> = s.split(" ").collect();

            println!("Conversion numbers {:?}", nums);

            let deststart = nums[0].parse::<u64>().unwrap();
            let srcstart = nums[1].parse::<u64>().unwrap();
            let rangelen = nums[2].parse::<u64>().unwrap();

            let mut j = 0;
            while j < mappings[srcmapno].len() {
                println!("Checking {:?}", mappings[srcmapno][j]);
                if mappings[srcmapno][j] >= srcstart && mappings[srcmapno][j] < srcstart + rangelen
                {
                    let val = mappings[srcmapno].remove(j);
                    println!(
                        "Matches mapping, mapped number {:?}",
                        val - srcstart + deststart
                    );

                    mappings[srcmapno + 1].push(val - srcstart + deststart);
                } else {
                    j += 1;
                };
            }
        }

        // Move any remaining

        let mut j = 0;
        while j < mappings[srcmapno].len() {
            let val = mappings[srcmapno][j];
            mappings[srcmapno + 1].push(val);
            j += 1;
        }

        println!("Current generation {:?}", mappings[srcmapno]);
        println!("Next generation {:?}", mappings[srcmapno + 1]);
    }
    mappings[7].sort();
    println!("{:?}", mappings[7]);
}
