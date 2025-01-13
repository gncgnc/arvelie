use chrono::{Datelike, NaiveDate};

pub fn to_arvelie_month_day(date: NaiveDate) -> String {
    let month_num = date.ordinal() / 14;
    let month_letter = if month_num < 26 {
        std::char::from_u32(month_num + 'A' as u32).unwrap()
    } else {
        '+'
    };

    let day_num = (date.ordinal() - 1) % 14;
    format!("{}{:02}", month_letter, day_num)
}

pub fn from_arvelie_month_day(arv: &str, year: i32) -> Result<NaiveDate, String> {
    if !arv.is_ascii() {
        return Err("lol not ascii".to_string());
    }

    match arv.as_bytes() {
        [month, day0, day1] => {
            let day0 = day0 - b'0';
            let day1 = day1 - b'0';

            let day = ((day0 as u32) * 10) + day1 as u32;
            let month = if *month != b'+' {
                (*month - b'A') as u32
            } else {
                26
            };
            let ordinal = day + month * 14 + 1;
            Ok(NaiveDate::from_yo_opt(year, ordinal)
                .ok_or(format!("lol out bounds - month = {month}, day = {day}"))?)
        }
        _ => Err("lol no".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO test errors

    fn check_roundtrip(arvelie_date: &str, year: i32, month: u32, day: u32) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let md = to_arvelie_month_day(date);
        assert_eq!(md, arvelie_date);

        let back = from_arvelie_month_day(&md, year).unwrap();
        assert_eq!(back, date, "got {}, expected {}", back, date);
    }

    #[test]
    fn extra_year_day() {
        // +01 2020-12-31
        check_roundtrip("+01", 2020, 12, 31);
    }

    #[test]
    fn year_day() {
        //
        check_roundtrip("+00", 2019, 12, 31);
    }

    #[test]
    fn two_k_seven_feb_18() {
        // D06 2007-02-18
        check_roundtrip("D06", 2007, 2, 18);
    }

    #[test]
    fn first_day_of_year() {
        // A00 2008-01-01
        check_roundtrip("A00", 2008, 1, 1);
    }

    #[test]
    fn random_date() {
        // F10 2009-03-22
        check_roundtrip("F10", 2009, 3, 22);
    }

    #[test]
    fn today() {
        // A12 2025-01-13
        check_roundtrip("A12", 2025, 1, 13);
    }
}
