use crate::model::{Category, Product, ProductInsert};
use chrono;
use std::collections::HashSet;
use surrealdb::engine::any::Any;

use surrealdb::{Error, Surreal};

pub async fn create_products(products: Vec<Product>, db: &Surreal<Any>) -> Result<(), Error> {
    for product in products {
        let category_key: Option<Category> =
            db.select(("category", &product.category.name)).await?;

        let _: Option<ProductInsert> = db
            .create("product")
            .content(ProductInsert {
                name: product.name,
                qty: product.qty,
                price: product.price,
                category: category_key.expect("Category not found").id.unwrap(),
                date: surrealdb::sql::Datetime::from(chrono::Utc::now()),
            })
            .await?;
    }
    Ok(())
}

pub async fn create_categories(
    categories: HashSet<Category>,
    db: &Surreal<Any>,
) -> Result<(), Error> {
    for category in categories {
        let _: Option<Category> = db
            .upsert(("category", &category.name))
            .content(category)
            .await?;
    }
    println!("Categories created");
    Ok(())
}
