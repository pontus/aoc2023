use std::fs::File;
use std::io::read_to_string;
use std::vec::Vec;

fn area_inside(map: &Vec<Vec<u8>>, border: &Vec<(usize, usize)>, (w, h): (usize, usize)) -> usize {
    let mut inside = 0;

    println!("Getting area inside border {:?}", border);

    for i in 0..w - 1 {
        for j in 0..h - 1 {
            if border.contains(&(i, j)) {
                //println!("{i},{j} is on the border, skipping");
                continue;
            }

            //println!("Checking if {i},{j} is inside");
            let mut counter = 0;
            let mut counters = 0;
            let mut c = (i + 1, j);

            while c.0 < w {
                if border.contains(&c) {
                    if map[c.1][c.0] == b'L' || map[c.1][c.0] == b'F' {
                        while map[c.1][c.0] != b'7' && map[c.1][c.0] != b'J' {
                            println!("Checking if {:?}", c);

                            c.0 += 1;
                            counter += 1;
                        }
                    } else {
                        println!("  {:?},{:?} is on the border", c.0, c.1);
                        counter += 1;
                    }
                }
                c.0 += 1;
            }
            println!(" Counters are {counter} going right, {counters} going left");

            if counter % 2 == 1 {
                println!("* {i},{j} saw the border {counter} times is inside");
                inside += 1;
            }
        }
    }

    return inside;
}

fn get_farthest(
    map: &Vec<Vec<u8>>,
    visited: Vec<(usize, usize)>,
    (x, y): (usize, usize),
    (dx, dy): (i8, i8),
) -> usize {
    // dx,dy how we came here, visited places before
    let mut nextv = visited.clone();
    nextv.push((x, y));
    let c = map[y][x];

    // Follow the matrix, higher x to the right, higher y downwards
    //println!("Called at {x},{y} with direction {dx},{dy} char is {c}");

    if c == b'S' {
        println!("Goal, returning {:?}", visited.len());

        return area_inside(map, &nextv, (map[0].len(), map.len()));
    }

    if c == b'.'
        || ((c == b'L' || c == b'F' || c == b'|') && dx == 1)
        || ((c == b'7' || c == b'J' || c == b'|') && dx == -1)
        || ((c == b'L' || c == b'J' || c == b'-') && dy == -1)
        || ((c == b'F' || c == b'7' || c == b'-') && dy == 1)
    {
        return 0;
    }

    let mut ny: usize;
    let mut nx: usize;
    if c == b'|' && dy != 0 {
        ny = (y as isize + dy as isize) as usize;
        return get_farthest(map, nextv, (x, ny), (dx, dy));
    }
    if c == b'-' && dx != 0 {
        nx = (x as isize + dx as isize) as usize;
        return get_farthest(map, nextv, (nx, y), (dx, dy));
    }

    if c == b'L' && dy == 1 {
        return get_farthest(map, nextv, (x + 1, y), (1, 0));
    }
    if c == b'L' && dx == -1 {
        return get_farthest(map, nextv, (x, y - 1), (0, -1));
    }

    if c == b'J' && dy == 1 {
        return get_farthest(map, nextv, (x - 1, y), (-1, 0));
    }
    if c == b'J' && dx == 1 {
        return get_farthest(map, nextv, (x, y - 1), (0, -1));
    }

    if c == b'F' && dy == -1 {
        return get_farthest(map, nextv, (x + 1, y), (1, 0));
    }
    if c == b'F' && dx == -1 {
        return get_farthest(map, nextv, (x, y + 1), (0, 1));
    }

    if c == b'7' && dy == -1 {
        return get_farthest(map, nextv, (x - 1, y), (-1, 0));
    }
    if c == b'7' && dx == 1 {
        return get_farthest(map, nextv, (x, y + 1), (0, 1));
    }

    println!("c is {c}");
    return 0;
}

fn main() {
    let reader = read_to_string(File::open("input.txt").unwrap()).unwrap();

    let mut sum = 0;

    let mut map: Vec<Vec<u8>> = Vec::new();

    for (linenumber, l) in reader.lines().enumerate() {
        let thisline = l.as_bytes();
        let ll = thisline.len();
        let mut v: Vec<u8> = Vec::new();

        for i in 0..ll {
            v.push(thisline[i]);
        }

        map.push(v);
    }

    for (j, v) in map.iter().enumerate() {
        for (i, w) in v.iter().enumerate() {
            if *w == b'S' {
                println!("Start is at {i},{j}");
                if (i > 0) {
                    println!(
                        "{:?}",
                        get_farthest(&map, Vec::<(usize, usize)>::new(), (i - 1, j), (-1, 0))
                    );
                }
                println!("Start is at {i},{j}");
                if (i < v.len() - 1) {
                    println!(
                        "{:?}",
                        get_farthest(&map, Vec::<(usize, usize)>::new(), (i + 1, j), (1, 0))
                    );
                }
                println!("Start is at {i},{j}");
                if (j > 0) {
                    println!(
                        "{:?}",
                        get_farthest(&map, Vec::<(usize, usize)>::new(), (i, j - 1), (0, -1))
                    );
                }
                println!("Start is at {i},{j}");
                if (j < map.len() - 1) {
                    println!(
                        "{:?}",
                        get_farthest(&map, Vec::<(usize, usize)>::new(), (i, j + 1), (0, 1))
                    );
                }
            }
        }
    }

    println!("{:?}", sum);
}
