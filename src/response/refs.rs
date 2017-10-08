#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RefsLocalResponse {
    #[serde(rename = "Ref")]
    pub reference: String,

    pub err: String,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_refs_local_0, RefsLocalResponse);
}
