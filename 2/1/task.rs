use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let max : HashMap<&str,u64> = HashMap::<&str,u64>::from([("red",12),("green",13), ("blue",14)]);
    let mut sum = 0;

    for l in lines {
        let s = l.unwrap();
        let v: Vec<&str> = s.split(":").collect();
        let s = v[1].split(";");

        let gamebit : Vec<&str> = v[0].split(" ").collect();
        let game = gamebit[1].parse::<u64>().unwrap();
        let mut possible =true;

        for draw in s {
            let part = draw.split(",");
        
            for p in part {
                let  spec : Vec<&str> = p.split(" ").collect();

                let color = spec[2];
                let num = spec[1].parse::<u64>().unwrap();                

                if num > *max.get(color).unwrap() {
                    possible= false;
                    println!("Bad case: {num} {color}")
                };
            }
        }

        if possible {
            sum += game;
        }


    }

    println!("{:?}", sum);


}
