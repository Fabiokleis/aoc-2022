pub mod day1;
pub mod day2;
pub mod day3;

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

    #[test]
    fn day3() {
        use crate::day3;
        day3::day3();
    }
}
