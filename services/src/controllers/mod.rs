mod clients;
mod exercises;
mod media;
mod trainers;

use serde::{Deserialize, Serialize};

pub use self::clients::*;
pub use self::exercises::*;
pub use self::media::*;
pub use self::trainers::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    code: u16,
    data: T,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}
