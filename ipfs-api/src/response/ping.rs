#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PingResponse {
    pub success: bool,
    pub time: i64,
    pub text: String,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_ping_0, PingResponse);
    deserialize_test!(v0_ping_1, PingResponse);
    deserialize_test!(v0_ping_2, PingResponse);
}
