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

    #[serde(default)]
    pub subcommands: Vec<CommandsResponse>,

    #[serde(default)]
    pub options: Vec<CommandsResponseOptions>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_commands_0, CommandsResponse);
}
