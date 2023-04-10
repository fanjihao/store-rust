use chrono::NaiveDate as NativeDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Foods {
    pub id: Option<u32>,
    pub name: String,
    pub photo: String,
    pub upload_date: String,
}
