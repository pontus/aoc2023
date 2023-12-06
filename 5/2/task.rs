use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn main() {
    let mut reader = io::BufReader::new(File::open("input.txt").unwrap());
    let mut lines = (&mut reader).lines();

    let seedline = lines.next().unwrap().unwrap();
    let seedlineparts: Vec<&str> = seedline.split(":").collect();

    let seedsstrings: Vec<&str> = seedlineparts[1].split(" ").collect();
    let mut mappings: Vec<Vec<(u64, u64)>> = Vec::new();

    for p in 0..10 {
        mappings.push(Vec::new());
    }

    println!("Seeing seed strings {:?}", seedsstrings);

    let mut j = 0;

    while j < seedsstrings.len() {
        let p = seedsstrings[j];

        if p.eq("") {
            j += 1;
            continue;
        }

        let q = seedsstrings[j + 1];

        let seedbase = p.parse::<u64>().unwrap();
        let seedrange = q.parse::<u64>().unwrap();
        println!("Seeing seed {seedbase}, {seedrange}");

        mappings[0].push((seedbase, seedrange));

        j += 2;
    }

    let startmappings = mappings[0].clone();

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
                println!("Current check src={srcstart} range={rangelen} dest={deststart}");

                let current = mappings[srcmapno][j];
                if current.0 + current.1 <= srcstart || current.0 >= srcstart + rangelen {
                    println!("Completely outside");
                    // Outside of ra.nge
                    j += 1;
                    continue;
                }

                if current.0 >= srcstart && current.0 + current.1 <= srcstart + rangelen {
                    println!("Completely inside");

                    // Completely inside
                    let val = mappings[srcmapno].remove(j);
                    mappings[srcmapno + 1].push((current.0 - srcstart + deststart, current.1));
                    continue;
                }

                if current.0 < srcstart
                    && (current.0 + current.1) > srcstart
                    && (current.0 + current.1) <= (srcstart + rangelen)
                {
                    println!("Starts below and ends inside");

                    // Split and handle in this generation
                    let val = mappings[srcmapno].remove(j);
                    mappings[srcmapno].push((current.0, srcstart - current.0));
                    mappings[srcmapno].push((srcstart, current.1 - (srcstart - current.0)));
                    continue;
                }

                if current.0 >= srcstart && current.0 + current.1 > srcstart + rangelen {
                    println!("Starts inside and goes beyond");

                    // Split and handle in this generation
                    let val = mappings[srcmapno].remove(j);
                    mappings[srcmapno].push((current.0, srcstart + rangelen - current.0));
                    mappings[srcmapno].push((
                        srcstart + rangelen,
                        current.1 - (srcstart + rangelen - current.0),
                    ));
                    continue;
                }

                if current.0 < srcstart && current.0 + current.1 > srcstart + rangelen {
                    println!("Envelops range");

                    // Split and handle in this generation
                    let val = mappings[srcmapno].remove(j);
                    mappings[srcmapno].push((current.0, srcstart - current.0));
                    mappings[srcmapno].push((srcstart, rangelen));

                    mappings[srcmapno].push((
                        srcstart + rangelen,
                        current.0 + current.1 - (srcstart + rangelen),
                    ));
                    continue;
                }

                // if current.0 < srcstart && current.0 + current.1 > srcstart && current.0 + current.1 < srcstart + rangelen {
                //     println!("Starts below and ends inside");

                //     // Completely inside
                //     let val = mappings[srcmapno].remove(j);
                //     mappings[srcmapno + 1].push((current.0 - srcstart + deststart, current.1));
                //     continue;
                // }

                println!("Not handled");
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
    println!("End generation {:?}", mappings[7]);

    println!("Seeing seed strings {:?}", seedlineparts);

    for p in &mappings[7] {
        if startmappings.contains(&p) {
            println! {"Found {:?}",p};
            break;
        }
    }
}
