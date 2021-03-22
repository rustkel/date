mod date;

fn main() {
    let d0 = date::Date::new(2000,2, 29);
    let d = date::Date::new(2001,2, 28);
    let g = date::Date::days_between_dates(&d0, &d);
    match g {
        Ok(days) => println!("{} - {}: {} days", d0, d, days),
        Err(e) => println!("{}", e)
    }
}
