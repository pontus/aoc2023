use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut sum = 0;

    for l in lines {
        let s = l.unwrap();
        let b = s.as_bytes();
 
        let mut i = 0;

        let mut first = 0;
        let mut last = 0;
        
        while i <b.len() {
            let c = b[i];
            if c>=48 && c<=57 {
                first = (c-48) as u64;
                break;
            }
            i+=1;
        }

        i= b.len();
        while i >0 {
            let c = b[i-1];
            if c>=48 && c<=57 {
                last = (c-48) as u64;
                break;
            }
            i-=1;
        }
        let this = first*10+last;
        println!("{:?}", this);

        sum += this;

    }

    println!("{:?}", sum);
}
