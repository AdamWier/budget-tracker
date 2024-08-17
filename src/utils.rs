use chrono::{Datelike, Local, NaiveDate};

pub fn parse_european_number_format(value: &str) -> Result<f32, String> {
    let error = format!("Error parsing amount {}", value);
    let parts: Vec<&str> = value.split(',').collect();
    let main_part = parts
        .first()
        .expect(&error)
        .replace('.', "")
        .parse::<f32>()
        .expect(&error);
    let decimal_part = parts.get(1).expect(&error).parse::<f32>().expect(&error) / 100.0;
    Ok(main_part + decimal_part)
}

pub fn get_days_in_current_month() -> u32 {
    NaiveDate::from_ymd_opt(Local::now().year(), Local::now().month(), 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day0()
        + 1
}
