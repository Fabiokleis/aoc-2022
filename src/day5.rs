use std::fs;
use regex::Regex;
use std::collections::VecDeque;
use std::collections::HashMap;

pub fn day5() {
    let contents = fs::read_to_string("./input5.txt").unwrap();
    /*let contents = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";*/

    let mut hash_stack: HashMap<usize, VecDeque<String>> = parse_stack_of_crates(contents.to_string());
    //println!("{:#?}", hash_stack);

    let number = Regex::new(r"\d{1,2}").unwrap();
    for l in contents.lines() {
        let mut instruction: Vec<usize> = vec![];
        if l.starts_with("move") {
            for c in number.find_iter(l) {
                instruction.push(c.as_str().parse::<usize>().unwrap());
            }

            for _times in 0..instruction[0] {
                let crat = hash_stack.get_mut(&instruction[1]).unwrap().pop_front();
                hash_stack.get_mut(&instruction[2]).unwrap().push_front(crat.unwrap());
            }
            
        }
    }

    let mut message: String = String::new();
    for i in 1..hash_stack.len()+1 {
        message.push_str(hash_stack.get(&i).unwrap().front().unwrap());
    }
    
    //println!("{:#?}", hash_stack);
    println!("{}", message);

    day5_part2(contents.to_string());
}

fn day5_part2(contents: String) {
    let mut hash_stack: HashMap<usize, VecDeque<String>> = parse_stack_of_crates(contents.to_string());
    //println!("{:#?}", hash_stack);

    let number = Regex::new(r"\d{1,2}").unwrap();
    for l in contents.lines() {
        let mut instruction: Vec<usize> = vec![];
        if l.starts_with("move") {
            for c in number.find_iter(l) {
                instruction.push(c.as_str().parse::<usize>().unwrap());
            }

            let mut crates: Vec<String> = vec![];
            for _times in 0..instruction[0] {
                let crat = hash_stack.get_mut(&instruction[1]).unwrap().pop_front().unwrap();
                crates.push(crat);
            }
            for crat in crates.iter().rev() {
                hash_stack.get_mut(&instruction[2]).unwrap().push_front(crat.clone());
            }
        }
    }

    let mut message: String = String::new();
    for i in 1..hash_stack.len()+1 {
        message.push_str(hash_stack.get(&i).unwrap().front().unwrap());
    }
    
    //println!("{:#?}", hash_stack);
    println!("{}", message);
}


fn parse_stack_of_crates(contents: String) -> HashMap<usize, VecDeque<String>> {
    
    let mut table: HashMap<usize, VecDeque<String>> = HashMap::new();
    let columns: String = contents.lines()
        .filter(|l| l.trim().starts_with('1')).collect();

    let mut indexs: Vec<usize> = vec![];
    for c in columns.chars() {
        if c.is_digit(10) {
            indexs.push(c.to_digit(10).unwrap() as usize);
        }
    }
    for i in indexs.iter() {
        table.insert(*i, VecDeque::new());
    }

    let crat = Regex::new(r"\[[a-zA-Z]{1}\]").unwrap();

    contents.lines().for_each(|l| {
       for c in crat.find_iter(l) {
           let s = c.start();
           let idx = columns.chars().nth(s+1).unwrap().to_digit(10).unwrap() as usize;
               let cratee = c.as_str().to_string().replace("[", "").replace("]", "");
           table.get_mut(&idx).unwrap().push_back(cratee);
        }
    });
    table
}
