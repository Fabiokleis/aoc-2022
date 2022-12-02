// A => Rock => X
// B => Paper => Y
// C => Scissors => Z


// A vs Y => 2 + 6 = 8
// A vs X => 1 + 3 = 4
// A vs Z => 3 + 0 = 3
//
// B vs Y => 2 + 3 = 5
// B vs X => 1 + 0 = 1
// B vs Z => 3 + 6 = 9
//
// C vs Y => 2 + 0 = 2
// C vs X => 1 + 6 = 7
// C vs Z => 3 + 3 = 6


// X => lose
// Y => draw
// Z => win



pub fn day2() {
    use std::fs;
    let contents = fs::read_to_string("./input2.txt").unwrap();
    //let contents = String::from("A Y\nB X\nC Z");


    let mut sum: u32 = 0;
    let mut out = String::from("");
    for l in contents.lines() {
        for c in l.trim().chars() {
            match c {
                ' ' => {},
                _ => {
                    out.push(c);
                }
            }
        }

        if out.starts_with("A") {
            if out.ends_with("Y") {
                sum += 8; 
            } else if out.ends_with("X") {
                sum += 4; 
            } else if out.ends_with("Z") {
                sum += 3; 
            }
        } else if out.starts_with("B") {
            if out.ends_with("Y") {
                sum += 5; 
            } else if out.ends_with("X") {
                sum += 1; 
            } else if out.ends_with("Z") {
                sum += 9; 
            }
        } else if out.starts_with("C") {
            if out.ends_with("Y") {
                sum += 2; 
            } else if out.ends_with("X") {
                sum += 7; 
            } else if out.ends_with("Z") {
                sum += 6; 
            }
        }
        out = String::from("");
    }
    println!("{}", sum);
    day2_part2(contents);
}

pub fn day2_part2(contents: String) {
    let mut sum: u32 = 0;
    let mut out = String::from("");
    for l in contents.lines() {
        for c in l.trim().chars() {
            match c {
                ' ' => {},
                _ => {
                    out.push(c);
                }
            }
        }

        if out.starts_with("A") {
            if out.ends_with("Y") {
                sum += 4; // 1 + 3
            } else if out.ends_with("X") {
                sum += 3; // 3 + 0
            } else if out.ends_with("Z") {
                sum += 8; // 2 + 6
            }
        } else if out.starts_with("B") {
            if out.ends_with("Y") {
                sum += 5; // 2 + 3
            } else if out.ends_with("X") {
                sum += 1; // 1 + 0
            } else if out.ends_with("Z") {
                sum += 9; // 3 + 6
            }
        } else if out.starts_with("C") {
            if out.ends_with("Y") {
                sum += 6; // 3 + 3
            } else if out.ends_with("X") {
                sum += 2; // 2 + 0
            } else if out.ends_with("Z") {
                sum += 7; // 1 + 6
            }
        }
        out = String::from("");
    }
    println!("{}", sum);
}
