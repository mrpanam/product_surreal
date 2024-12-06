use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub qty: i64,
    pub price: f64,
    pub category: Category,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Category {
    pub id: Option<RecordId>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductInsert {
    pub name: String,
    pub qty: i64,
    pub price: f64,
    pub category: RecordId,
    pub date: Datetime,
}
