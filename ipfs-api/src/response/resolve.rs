#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResolveResponse {
    pub path: String,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_resolve_0, ResolveResponse);
}
