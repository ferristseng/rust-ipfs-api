#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TarAddResponse {
    pub name: String,
    pub hash: String,
    pub size: String,
}


pub type TarCatResponse = Vec<u8>;


#[cfg(test)]
mod tests {
    deserialize_test!(v0_tar_add_0, TarAddResponse);
}
