use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Country {
    pub name: String,
    pub flag: String,
    pub independent: bool,
}
