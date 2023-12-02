use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut sum = 0;

    for l in lines {
        let s = l.unwrap();
        let v: Vec<&str> = s.split(":").collect();
        let s = v[1].split(";");

        let mut seen : HashMap<&str,u64> = HashMap::<&str,u64>::from([("red",0),("green",0), ("blue",0)]);

        for draw in s {
            let part = draw.split(",");
        
            for p in part {
                let  spec : Vec<&str> = p.split(" ").collect();

                let color = spec[2];
                let num = spec[1].parse::<u64>().unwrap();                

                if num > *seen.get(color).unwrap() {

                    seen.insert(color, num);
                    //println!("Bad case: {num} {color}")
                };
            }
        }

        let power = (*seen.get("red").unwrap())*(*seen.get("blue").unwrap())*(*seen.get("green").unwrap());
        sum+= power;
    }

    println!("{:?}", sum);


}
