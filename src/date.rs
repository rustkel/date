//Date utils
//Mostly for day counting between dates

//There are many ways to count dates; here I have chosen:
//- Gregorian date from 1582 AD onwards
//- Julian date before that (Proleptic Julian Calendar)
//- No year 0
//It is more interesting from a computational point of view!

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

    pub fn is_valid(&self) -> Result<(), String> {
        match (self.year, self.month, self.day) {
            (0, _, _) => Err("Year 0 does not exist".into()),
            (_, month, _) if month < 1 || month > 12 => Err("Invalid month".into()),
            (GREGORIAN_YEAR, 10, day) if day > 4 && day < 14 
                => Err(format!("{} does not exist", self)),
            (year, month, day) => {
                let mut m = MONTH_DAYS[month as usize - 1];
                if month == 2 {
                    m += if Date::is_leap(year) { 1 } else { 0 }
                }
                if day > 0 && day <= m as u8 {
                    Ok(())
                } else {
                    Err(format!("{}: Invalid day", self))
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

    pub fn day_of_year(&self) -> i32 {
        let mut days = Date::month_days(self.month, self.year) as i32;
        if self.year == GREGORIAN_YEAR && (self.month > 10
            || (self.month == 10 && self.day > 13) ) {
            days -= 10;
        }
        days + self.day as i32
    }

    pub fn days_between_dates(first: &Date, last: &Date) -> Result<i32, String> {
        if let Err(error) = first.is_valid() {
            return Err(error);
        }
        if let Err(error) = last.is_valid() {
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
        let d1 = first.day_of_year();
        let d2 = last.day_of_year();
        if first.year > last.year {
            Ok(-days - d1 + d2)
        } else {
            Ok(days - d1 + d2)
        }
    }
}

#[test]
fn test_leap_years() {
    assert_eq!(Date::is_leap(2000),true);
    assert_eq!(Date::is_leap(2001),false);
    assert_eq!(Date::is_leap(2100),false);
    assert_eq!(Date::is_leap(2004),true);
    assert_eq!(Date::is_leap(100),true);
    assert_eq!(Date::is_leap(-1),true);
    assert_eq!(Date::is_leap(-4),false);
}

#[test]
fn test_invalid_dates() {
    let date = Date::new(1979, 2, 29);
    assert!(date.is_valid().is_err());

    let date = Date::new(1979, 1, 0);
    assert!(date.is_valid().is_err());

    let date = Date::new(0, 1, 1);
    assert!(date.is_valid().is_err());

    let date = Date::new(1, 1, 41);
    assert!(date.is_valid().is_err());

    let date = Date::new(-4, 15, 1);
    assert!(date.is_valid().is_err());

    let date = Date::new(1582, 10, 11);
    assert!(date.is_valid().is_err());
}

#[test]
fn test_days_in_year() {
    assert_eq!(Date::year_days(1977),365);
    assert_eq!(Date::year_days(1978),365);
    assert_eq!(Date::year_days(1980),366);
    assert_eq!(Date::year_days(2000),366);
    assert_eq!(Date::year_days(2100),365);
    assert_eq!(Date::year_days(1582),355);
    assert_eq!(Date::year_days(0),0);
    assert_eq!(Date::year_days(-1),366);
}

#[test]
fn test_days_between_dates() {
    let first = Date::new(1950, 1, 1);
    let last = Date::new(1950, 12, 31);
    assert_eq!(Date::days_between_dates(&first, &last), Ok(364));

    let first = Date::new(1980, 1, 1);
    let last = Date::new(1981, 1, 1);
    assert_eq!(Date::days_between_dates(&first, &last), Ok(366));
    assert_eq!(Date::days_between_dates(&last, &first), Ok(-366));

    let first = Date::new(1977, 10, 1);
    let last = Date::new(2021, 7, 22);
    assert_eq!(Date::days_between_dates(&first, &last), Ok(16000));


    let first = Date::new(2000, 1, 1);
    let last = Date::new(2400, 1, 1);
    assert_eq!(Date::days_between_dates(&first, &last), Ok(146097));

    let first = Date::new(2000, 1, 1);
    let last = Date::new(2400, 1, 1);
    assert_eq!(Date::days_between_dates(&first, &last), Ok(146097));

    let first = Date::new(1582, 1, 1);
    let last = Date::new(1982, 1, 1);
    assert_eq!(Date::days_between_dates(&first, &last), Ok(146087));

    let first = Date::new(0, 1, 1);
    assert!(Date::days_between_dates(&first, &last).is_err());

}