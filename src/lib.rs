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
            Ok(NaiveDate::from_ymd_opt(year, 1, 1)
                .unwrap()
                .with_ordinal(day + month * 14 + 1)
                .ok_or(format!("lol out bounds - month = {month}, day = {day}"))?)
        }
        _ => Err("lol no".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO test errors

    #[test]
    fn extra_year_day() {
        // 14+01 2020-12-31
        let date = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap();
        let md = to_arvelie_month_day(date);
        assert_eq!(md, "+01");

        let back = from_arvelie_month_day(&md, 2020).unwrap();
        assert_eq!(back, date, "got {}, expected {}", back, date);
    }

    #[test]
    fn two_k_seven_feb_18() {
        // 01D06 2007-02-18
        let date = NaiveDate::from_ymd_opt(2007, 2, 18).unwrap();
        let md = to_arvelie_month_day(date);
        assert_eq!(md, "D06");

        let back = from_arvelie_month_day(&md, 2007).unwrap();
        assert_eq!(back, date, "got {}, expected {}", back, date);
    }

    #[test]
    fn first_day_of_year() {
        // 02A00 2008-01-01
        let date = NaiveDate::from_ymd_opt(2008, 1, 1).unwrap();
        let md = to_arvelie_month_day(date);
        assert_eq!(md, "A00");

        let back = from_arvelie_month_day(&md, 2008).unwrap();
        assert_eq!(back, date, "got {}, expected {}", back, date);
    }

    #[test]
    fn random_date() {
        // 11F10 2009-03-22
        let date = NaiveDate::from_ymd_opt(2009, 3, 22).unwrap();
        let md = to_arvelie_month_day(date);
        assert_eq!(md, "F10");

        let back = from_arvelie_month_day(&md, 2009).unwrap();
        assert_eq!(back, date, "got {}, expected {}", back, date);
    }
}
