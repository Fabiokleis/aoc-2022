#![feature(str_split_as_str, string_remove_matches)]
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

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

    #[test]
    fn day4() {
        use crate::day4;
        day4::day4();
    }

    #[test]
    fn day5() {
        use crate::day5;
        day5::day5();
    }
}
