use core::cmp::Ordering;
use std::fs::File;
use std::io::read_to_string;
use std::vec::Vec;

fn type_hand(a: &str) -> usize {
    let mut bestseen = 0;
    let ab = a.as_bytes();

    println!("Checking type of hand for {a} ");

    let mut v: Vec<usize> = Vec::new();
    v.resize(15, 0);

    for i in 0..5 {
        // i is startindex;
        v[card_value(ab[i])] += 1;
    }

    let jokers = v[1];
    v[1] = 0;

    v.sort();
    v.reverse();
    println!("Sorted as {:?}", v);

    v[0] += jokers;

    if v[0] == 5 {
        println!("Returning 10 ");

        return 10;
    }
    if v[0] == 4 {
        println!("Returning 9 ");
        return 9;
    }
    if v[0] == 3 && v[1] == 2 {
        println!("Returning 8 ");

        return 8;
    }
    if v[0] == 3 {
        println!("Returning 7 ");

        return 7;
    }
    if v[0] == 2 && v[1] == 2 {
        println!("Returning 6 ");
        return 6;
    }
    if v[0] == 2 {
        println!("Returning 5 ");

        return 5;
    }

    println!("Returning 4 ");

    return 4;
}

fn card_value(a: u8) -> usize {
    let val = match a {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 1,
        b'T' => 10,
        b'9' => 9,
        b'8' => 8,
        b'7' => 7,
        b'6' => 6,
        b'5' => 5,
        b'4' => 4,
        b'3' => 3,
        b'2' => 2,
        _ => 30,
    };

    println!("Card value for {a} is {val}");
    return val;
}

fn compare_hands(ah: &(&str, usize), bh: &(&str, usize)) -> Ordering {
    let a = ah.0;
    let b = bh.0;

    println!("Checking {:?} vs {:?}", a, b);
    if type_hand(a) != type_hand(b) {
        return type_hand(a).cmp(&type_hand(b));
    }

    for i in 0..5 {
        if card_value(a.as_bytes()[i]) != card_value(b.as_bytes()[i]) {
            return card_value(a.as_bytes()[i]).cmp(&card_value(b.as_bytes()[i]));
        }
    }

    return Ordering::Equal;
}

fn main() {
    let reader = read_to_string(File::open("input.txt").unwrap()).unwrap();

    let mut sum = 0;

    let mut hands: Vec<(&str, usize)> = Vec::new();

    for (_linenumber, l) in reader.lines().enumerate() {
        let lineparts: Vec<&str> = l.split(" ").collect();
        println!("{:?}", l);

        let bid = lineparts[1].parse::<usize>().unwrap();
        hands.push((lineparts[0], bid));
    }

    hands.sort_by(compare_hands);

    let mut rank = 0;
    while rank < hands.len() {
        let bet = hands[rank].1;
        rank += 1;
        println!("{rank} {bet} {:?}", rank * bet);

        sum += rank * bet;
    }

    println!("{:?}", hands);
    println!("{:?}", sum);
}
