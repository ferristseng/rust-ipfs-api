#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NamePublishResponse {
    pub name: String,
    pub value: String
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameResolveResponse {
    pub path: String
}


#[cfg(test)]
mod tests {
    use super::NameResolveResponse;


    deserialize_test!(v0_name_resolve_0, NameResolveResponse);
}
