use std::collections::HashMap;


pub type ObjectDataResponse = Vec<u8>;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectDiff {
    #[serde(rename = "Type")]
    typ: isize,

    path: String,

    #[serde(default)]
    before: HashMap<String, String>,

    #[serde(default)]
    after: HashMap<String, String>
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectDiffResponse {
    #[serde(default)]
    changes: Vec<ObjectDiff>
}


#[cfg(test)]
mod tests {
    use super::ObjectDiffResponse;


    deserialize_test!(v0_object_diff_0, ObjectDiffResponse);
}
