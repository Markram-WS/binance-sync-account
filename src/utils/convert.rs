use serde::Deserialize;

pub fn str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

pub fn vec_str_pair_to_f64<'de, D>(deserializer: D) -> Result<Vec<(f64, f64)>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<[String; 2]> = Vec::deserialize(deserializer)?;
    let mut result = Vec::with_capacity(raw.len());
    for pair in raw {
        let price = pair[0].parse::<f64>().map_err(serde::de::Error::custom)?;
        let qty = pair[1].parse::<f64>().map_err(serde::de::Error::custom)?;
        result.push((price, qty));
    }
    Ok(result)
}