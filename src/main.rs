use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();

    let result: i32 = 
        contents.split("\n\n")
        .map(|c| c.lines()
            .map(|l| l.parse::<i32>().unwrap()).sum()).max().unwrap();
    println!("{}", result);
}
