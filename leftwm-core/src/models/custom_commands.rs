use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Custom_cmds {
    pub name: String,
    pub args: Option<Vec<String>>,
    pub value: String,
}
