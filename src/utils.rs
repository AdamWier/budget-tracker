use std::num::ParseFloatError;

pub fn parse_european_number_format(value: &str) -> Result<f32, ParseFloatError> {
    value
        .chars()
        .filter(|&c| c != '.' && c != ' ')
        .map(|c| if c == ',' { '.' } else { c })
        .collect::<String>()
        .parse::<f32>()
}
