pub fn day1() {
    use std::fs;
    let contents = fs::read_to_string("./input.txt").unwrap();

    let mut result: Vec<i32> = 
        contents.split("\n\n")
        .map(|c| c.lines()
            .map(|l| l.parse::<i32>().unwrap()).sum::<i32>()).collect();

    result.sort();
    println!("{}", result.last().unwrap());
    let sum: i32 = result.iter().rev().take(3).sum();
    println!("{}", sum);
}
