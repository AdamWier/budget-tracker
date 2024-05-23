use serde::de::Error;
use serde::{Deserialize, Deserializer};

use crate::utils;

pub fn deserialize_amount<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer).expect("Cannot deserialize value");
    let parse_response = utils::parse_european_number_format(value);
    parse_response.map_err(Error::custom)
}
