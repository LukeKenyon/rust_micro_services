use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerCreated {
    pub id: String,
    pub name: String,
}
