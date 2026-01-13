use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub amount: i64,
    pub category: String,
    pub date: String,
    pub note: Option<String>,
}