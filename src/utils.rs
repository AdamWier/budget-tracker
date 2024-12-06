use std::num::ParseFloatError;

use chrono::{Datelike, Local, NaiveDate};

pub fn parse_european_number_format(value: &str) -> Result<f32, ParseFloatError> {
    value
        .chars()
        .filter(|&c| c != '.' && c != ' ')
        .map(|c| if c == ',' { '.' } else { c })
        .collect::<String>()
        .parse::<f32>()
}

pub fn get_days_in_current_month() -> Option<u32> {
    NaiveDate::from_ymd_opt(Local::now().year(), Local::now().month(), 1)
        .and_then(|x| x.pred_opt())
        .map(|x| x.day0() + 1)
}
