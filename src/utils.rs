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
