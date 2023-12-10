use core::cmp::Ordering;
use std::fs::File;
use std::io::read_to_string;
use std::vec::Vec;

fn predict(v: Vec<i64>) -> i64 {
    println!("{:?}", v);
    let mut n = &v[0];
    let mut same = true;
    for p in v.iter() {
        if p != n {
            same = false;
        }
    }

    if same {
        return *n;
    }

    let mut diffs: Vec<i64> = Vec::new();

    for p in v[1..].iter() {
        diffs.push(p - n);
        n = p;
    }

    return v.last().unwrap() + predict(diffs);
}

fn stringtovec(s: &str) -> Vec<i64> {
    let lineparts: Vec<&str> = s.split(" ").collect();
    println!("{:?}", s);

    let mut v: Vec<i64> = Vec::new();
    for p in lineparts {
        if p.eq("") {
            continue;
        }

        let n = p.parse::<i64>().unwrap();
        v.push(n);
    }

    return v;
}

fn main() {
    let reader = read_to_string(File::open("input.txt").unwrap()).unwrap();

    let mut sum = 0;

    for (_linenumber, l) in reader.lines().enumerate() {
        let x = predict(stringtovec(l));
        println!("Predict {x} for {l}",);

        sum += x;
    }

    println!("{:?}", sum);
}
