use response::serde;
use std::collections::HashMap;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinAddResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub pins: Vec<String>,

    pub progress: Option<isize>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinType {
    #[serde(rename = "Type")]
    pub typ: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinLsResponse {
    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub keys: HashMap<String, PinType>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinRmResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub pins: Vec<String>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_pin_ls_0, PinLsResponse);
}
