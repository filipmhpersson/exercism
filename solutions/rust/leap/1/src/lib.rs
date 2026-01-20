pub fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

// - In every year that is evenly divisible by 4.
// - Unless the year is evenly divisible by 100, in which case it's only a leap year if the year is also evenly divisible by 400.
