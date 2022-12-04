#![feature(str_split_as_str)]

pub fn day4() {
    use std::fs;
    let contents = fs::read_to_string("./input4.txt").unwrap();
    /*let contents = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
*/
    let result: Vec<(&str, &str)> = contents.lines()
        .map(|l| l.split_once(',').unwrap()).collect(); 

    //println!("{:#?}", result);
    let mut ranges: Vec<((u32, u32),(u32, u32))> = vec![];
    for (l, r) in result {
        let t = l.split_once('-').map(|c| (c.0.parse::<u32>().unwrap(), c.1.parse::<u32>().unwrap())).unwrap();
        let u = r.split_once('-').map(|c| (c.0.parse::<u32>().unwrap(), c.1.parse::<u32>().unwrap())).unwrap();
        ranges.push((t, u));
    }
    //println!("{:#?}", ranges);

    let mut counter = 0;
    for (l, r) in ranges.iter() {
        if (l.0 <= r.0 && l.1 >= r.1) || (r.0 <= l.0 && r.1 >= l.1) {
            counter += 1;
        }
    }
    println!("{}", counter);

    day4_part2(contents);
}

fn day4_part2(contents: String) {
    let result: Vec<(&str, &str)> = contents.lines()
        .map(|l| l.split_once(',').unwrap()).collect(); 

    //println!("{:#?}", result);
    let mut ranges: Vec<((u32, u32),(u32, u32))> = vec![];
    for (l, r) in result {
        let t = l.split_once('-').map(|c| (c.0.parse::<u32>().unwrap(), c.1.parse::<u32>().unwrap())).unwrap();
        let u = r.split_once('-').map(|c| (c.0.parse::<u32>().unwrap(), c.1.parse::<u32>().unwrap())).unwrap();
        ranges.push((t, u));
    }

    let mut overlaps = 0;
    for (l, r) in ranges.iter() {
        if (l.0 <= r.0 && r.0 <= l.1) || (r.0 <= l.0 && l.0 <= r.1) { overlaps += 1; }
    }
    println!("{}", overlaps);
}
