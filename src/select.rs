use crate::model::Category;
use surrealdb::engine::any::Any;

use surrealdb::{Error, Surreal};

pub async fn select_category(db: &Surreal<Any>) -> Result<Category, Error> {
    let category: Option<Category> = db.select(("category", "Fruits")).await?;

    match category {
        Some(cat) => {
            println!("Category: {:#?}", cat);
            Ok(cat)
        }
        None => Err(Error::Db(surrealdb::error::Db::NoRecordFound)),
    }
}
