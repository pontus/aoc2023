use std::fs::File;
use std::io::read_to_string;
use std::vec::Vec;

fn is_numeric(x: u8) -> bool {
    if x >= b'0' && x <= b'9' {
        return true;
    }
    return false;
}

fn get_number(x: &[u8], i: usize) -> (usize, u64) {
    let mut start = i;
    let mut end = i;

    while start > 0 && is_numeric(x[start]) {
        start -= 1;
    }

    // Start should point to first numeric
    if start != 0 || !is_numeric(x[0]) {
        start += 1;
    }

    while end < x.len() && is_numeric(x[end]) {
        end += 1;
    }

    let mut j = start;
    let mut val = 0;

    while j < end {
        val *= 10;
        val += (x[j] - 48) as u64;
        j += 1;
    }

    return (start, val);
}

fn main() {
    let reader = read_to_string(File::open("input.txt").unwrap()).unwrap();

    let mut sum = 0;

    let mut lastline: &[u8] = &[];
    let mut seen: Vec<((usize, usize), usize, usize, u64)> = Vec::new();

    for (linenumber, l) in reader.lines().enumerate() {
        let thislinesrc = l;
        let thisline = thislinesrc.as_bytes();
        let ll = thisline.len();

        for i in 0..ll - 1 {
            if thisline[i] == b'*' {
                //println!("Symbol at {i}  ");

                for check in [i - 1, i + 1] {
                    if check >= 0 && is_numeric(thisline[check]) {
                        let (start, val) = get_number(thisline, check);
                        if !seen.contains(&((linenumber, i), linenumber, start, val)) {
                            println!("Adding {val}, found at {linenumber},{start}");
                            seen.push(((linenumber, i), linenumber, start, val));
                        }
                    }
                }

                for check in [i - 1, i, i + 1] {
                    if linenumber > 0 && check >= 0 && is_numeric(lastline[check]) {
                        let (start, val) = get_number(lastline, check);
                        if !seen.contains(&((linenumber, i), linenumber - 1, start, val)) {
                            println!("Adding {val}, found at {linenumber}-1,{start}");
                            seen.push(((linenumber, i), linenumber - 1, start, val));
                        }
                    }
                }
            }

            if linenumber > 0 && lastline[i] == b'*' {
                //println!("Symbol at {i}  ");

                for check in [i - 1, i, i + 1] {
                    if check >= 0 && is_numeric(thisline[check]) {
                        let (start, val) = get_number(thisline, check);
                        if !seen.contains(&((linenumber - 1, i), linenumber, start, val)) {
                            println!("Adding {val}, found at {linenumber},{start}");
                            seen.push(((linenumber - 1, i), linenumber, start, val));
                        }
                    }
                }
            }
        }

        lastline = thisline;
    }

    for seenindex in 0..seen.len() {
        let pos = seen[seenindex].0;
        let mut val = seen[seenindex].3;

        println!("Checking pos {:?} index {seenindex}, val {val}", pos);
        let mut count = 0;
        for index in 0..seen.len() {
            if index != seenindex && pos == seen[index].0 {
                let ipos = seen[index].0;
                println!(
                    "Seen same pos {:?} index {index}, val {:?}",
                    seen[index].3, seen[index].3
                );

                count += 1;
                val *= seen[index].3;
            }
        }

        if count == 1 {
            println!("Adding {val} for {:?}, index {seenindex}", pos);

            sum += val;
        }
    }

    println!("{:?}", sum / 2);
}
