use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::{Error, RecordId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub qty: i64,
    pub price: f64,
    pub category: Category,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Category {
    pub id: RecordId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductInsert {
    pub name: String,
    pub qty: i64,
    pub price: f64,
    pub category: RecordId,
}

// Custom serializer for category reference
fn serialize_category_ref<S>(category_id: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde_json::json;
    let record = json!({
        "tb": "category",
        "id": category_id
    });
    record.serialize(serializer)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}
