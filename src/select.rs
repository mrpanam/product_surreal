use crate::model::{Category, Record};
use surrealdb::engine::any::Any;

use surrealdb::sql::Thing;
use surrealdb::{Error, Surreal};

pub async fn select_category(db: &Surreal<Any>) -> Result<Thing, Error> {
    let category: Thing = db.select(("category", "Fruits")).await?.unwrap();

    // Convert Option to Result

    println!("Category: {:#?}", category);

    Ok(category)
}
