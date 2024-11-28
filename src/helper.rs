use crate::model::{Category, Product, ProductInsert};
use std::collections::{HashMap, HashSet};
use surrealdb::engine::any::Any;

use surrealdb::{Error, Surreal};

pub async fn create_products(products: Vec<Product>, db: &Surreal<Any>) -> Result<(), Error> {
    for product in products {
        // Create or get the category Thing

        let category_key: Option<Category> =
            db.select(("category", &product.category.name)).await?;
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

pub async fn create_product(db: &Surreal<Any>) -> Result<(), Error> {
    let category_key: Option<Category> = db.select(("category", "Nuts")).await?;

    println!("Category key: {:#?}", category_key);

    let _: Option<ProductInsert> = db
        .create("product")
        .content(ProductInsert {
            name: "Coconut".to_string(),
            qty: 100,
            price: 5.45,
            category: category_key.expect("Category not found").id.unwrap(),
        })
        .await?;

    println!("Product created");
    Ok(())
}
