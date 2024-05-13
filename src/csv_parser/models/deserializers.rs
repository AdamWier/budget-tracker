use serde::{Deserialize, Deserializer};

pub fn deserialize_amount<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer).expect("Cannot deserialize value");
    let error = format!("Error parsing amount {}", value);

    let parts: Vec<&str> = value.split(',').collect();
    let main_part = parts.first().expect(&error).parse::<f32>().expect(&error);
    let decimal_part = parts.get(1).expect(&error).parse::<f32>().expect(&error) / 100.0;
    Ok(main_part + decimal_part)
}