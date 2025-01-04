use scylla::{frame::value::CqlTimestamp, DeserializeRow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, DeserializeRow)]
pub struct Client {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

#[derive(DeserializeRow, Clone, Debug)]
pub struct ClientsByIdRow {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,
}

#[derive(DeserializeRow, Clone, Debug)]
pub struct ClientsByEmailRow {
    pub client_id: String,
    pub email: String,
}
