use std::collections::HashMap;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinAddResponse {
    #[serde(default)]
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
    pub keys: HashMap<String, PinType>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinRmResponse {
    #[serde(default)]
    pub pins: Vec<String>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_pin_ls_0, PinLsResponse);
}
