use response::serde;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommandsResponseOptions {
    #[serde(default)]
    pub names: Vec<String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommandsResponse {
    pub name: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub subcommands: Vec<CommandsResponse>,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub options: Vec<CommandsResponseOptions>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_commands_0, CommandsResponse);
}
