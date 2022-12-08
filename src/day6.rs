fn start_of_packet_or_message(contents: &String, s: usize) -> usize {
    use std::collections::VecDeque;

    let tmp: VecDeque<char> = contents.chars().collect();

    let mut result = 0;
    let mut idx = 0;
    let mut st = 0;
    let mut f = false;

    'loopao: loop {
        if idx == s {
            if (st + idx) > tmp.len() { break 'loopao; }
            let tmp2: VecDeque<&char> = tmp.range(st..st+idx).collect();

            for (i, c) in tmp2.iter().enumerate() {
                for (i2, c2) in tmp2.iter().enumerate() {
                    if (c == c2) && (i != i2) {
                        f = true;
                    }
                }
            }
            if !f {
                result = st + idx;
                break 'loopao;
            } else {
                st += 1;
            }
            idx = 0;
            f = false;
        }
        idx += 1;
    }

    result
}

pub fn day6() {
    use std::fs;

    let contents = fs::read_to_string("./input6.txt").unwrap();
    //let contents = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    //println!("{}", contents);
    
    let result = start_of_packet_or_message(&contents, 4);
    println!("{}", result);
    day6_part2(&contents);
}

fn day6_part2(contents: &String) {
    //let contents = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    //println!("{}", contents);
    let result = start_of_packet_or_message(&contents, 14);
    println!("{}", result);
}
