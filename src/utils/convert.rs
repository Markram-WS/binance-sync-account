use serde::Deserialize;

pub fn i8_to_str<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: &i8 = &Deserialize::deserialize(deserializer)?;
    Ok(value.to_string())
}
pub fn i32_to_str<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: &i32 = &Deserialize::deserialize(deserializer)?;
    Ok(value.to_string())
}
pub fn i64_to_str<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: &i64 = &Deserialize::deserialize(deserializer)?;
    Ok(value.to_string())
}

pub fn str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}


pub fn str_to_option_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Deserialize เป็น Option<String> แทน &str
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            // แปลง string เป็น f64
            s.parse::<f64>()
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        None => Ok(None), // ไม่มีค่า → None
    }
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