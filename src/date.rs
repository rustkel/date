//Julian and Gregorian date utils
//Mostly for day counting between dates
//Assumes that year 0 is invalid
//Also that Julian calendar goes back in time
//beyond 45 BC when it was established

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let era = if self.year < 0 { " BC" } else { "" }; //less verbose than " AD"
        write!(
            f, "{} {}, {}{}", MONTHS[self.month as usize - 1],
            self.day, self.year.abs(), era)
    }
}

const GREGORIAN_YEAR: i32 = 1582;
static MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
static RUNNING_DAYS_PER_MONTH: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
pub static MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December"
];

impl Date {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn is_valid(date: &Date) -> Result<(), String> {
        match (date.year, date.month, date.day) {
            (0, _, _) => Err("Year 0 does not exist".into()),
            (_, month, _) if month < 1 || month > 12 => Err("Invalid month".into()),
            (GREGORIAN_YEAR, 10, day) if day > 4 && day < 14 
                => Err(format!("{} does not exist", date)),
            (year, month, day) => {
                let mut m = MONTH_DAYS[month as usize - 1];
                if month == 2 {
                    m += if Date::is_leap(year) { 1 } else { 0 }
                }
                if day > 0 && day <= m as u8 {
                    Ok(())
                } else {
                    Err(format!("{}: Invalid day", date))
                }
            }
        }
    }

    pub fn is_leap(year: i32) -> bool {
        let mut y = year;
        if y < 0 { y += 1; } //no year 0
        if y < GREGORIAN_YEAR { return y % 4 == 0; }
        y % 400 == 0 || (y % 4 == 0 && y % 100 != 0)
    }

    fn year_days(year: i32) -> i32 {
        if year == 0 { return 0 }
        if year == GREGORIAN_YEAR { return 355 }
        365 + if Date::is_leap(year) { 1 } else { 0 }
    }

    fn month_days(month: u8, year: i32) -> u32 {
        let mut bias = if month > 2 && Date::is_leap(year) { 1 } else { 0 };
        if year == GREGORIAN_YEAR && month > 9 { bias -= 10 }
        RUNNING_DAYS_PER_MONTH[month as usize - 1] + bias
    }

    pub fn day_of_year(date: &Date) -> i32 {
        let mut days = Date::month_days(date.month, date.year) as i32;
        if date.year == GREGORIAN_YEAR && (date.month > 10 
            || (date.month == 10 && date.day > 13) ) {
            days -= 10;
        }
        days + date.day as i32
    }

    pub fn days_between_dates(first: &Date, last: &Date) -> Result<i32, String> {
        if let Err(error) = Date::is_valid(&first) {
            return Err(error);
        }
        if let Err(error) = Date::is_valid(&last) {
            return Err(error);
        }

        let mut days = 0;
        let (year1, year2) = if first.year > last.year {
            (last.year, first.year)
        } else {
            (first.year, last.year)
        };
        for year in year1..year2 {
            days += Date::year_days(year);
        }
        let d1 = Date::day_of_year(&first);
        let d2 = Date::day_of_year(&last);
        if first.year > last.year {
            Ok(-days - d1 + d2)
        } else {
            Ok(days - d1 + d2)
        }
    }
}
