pub mod day1;
pub mod day2;

#[cfg(test)]
mod test {
    #[test]
    fn day1() {
        use crate::day1;
        day1::day1();
    }

    #[test]
    fn day2() {
        use crate::day2;
        day2::day2();
    }
}
