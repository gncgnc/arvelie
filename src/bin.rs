use arvelie::to_arvelie_month_day;
use chrono::Local;

pub fn main() {
    let today = Local::now().date_naive();
    println!("{}", to_arvelie_month_day(today));
}
