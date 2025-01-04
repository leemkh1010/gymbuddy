use scylla::DeserializeRow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, DeserializeRow)]
pub struct Exercise {}
