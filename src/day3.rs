use std::fs;
use std::collections::HashMap;

pub fn day3() {
    let contents = fs::read_to_string("./input3.txt").unwrap();
    
    /*let contents = String::from(
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    );*/
    
    let mut v: HashMap<u32, (&str, &str)> = HashMap::new(); 
    for (i, l) in contents.lines().enumerate() {
        v.insert(i as u32, l.split_at(l.len()/2));
    }

    //println!("{:#?}", v);
    let mut result: Vec<u8> = vec![];

    for (_,(l, r)) in v.iter() {
        for c in l.chars() {
            match r.find(c) {
                None => {},
                _ => { 
                    result.push(c as u8);
                    break;
                },
            }
        }
    }

    let mut sum: u32 = 0;
    for c in result.iter() {
        let mut a = 0;
        if (c.clone() as char).is_uppercase() {
            a = c - 65 + 27;
        } else if (c.clone() as char).is_lowercase() {
            a = c - 97 + 1;
        }
        sum += a as u32;
    }
    println!("{}", sum);
    day3_part2(contents);
}

fn day3_part2(contents: String) {

    let mut v: Vec<(&str, &str, &str)> = vec![];
 
    let mut t: Vec<&str> = vec![];
    for l in contents.lines() {
        t.push(l);
        if t.len() == 3 {
            v.push((t[0], t[1], t[2]));
            t = vec![];
        }
    }
    //println!("{:#?}", v);

    let mut result: Vec<u8> = vec![];
    for (r, u, t) in v.iter() {
        for c in r.chars() {
            if u.contains(c) && t.contains(c) {
                result.push(c as u8);
                break
            }
        }
    }
    //println!("{:#?}", result);

    let mut sum: u32 = 0;
    for c in result.iter() {
        let mut a = 0;
        if (c.clone() as char).is_uppercase() {
            a = c - 65 + 27;
        } else if (c.clone() as char).is_lowercase() {
            a = c - 97 + 1;
        }
        sum += a as u32;
    }
    println!("{}", sum);
}

