use surrealdb::engine::any::Any;

use surrealdb::{Error, Surreal};

pub async fn init_db(db: &Surreal<Any>) -> Result<(), Error> {
    /*db.query(
        "DEFINE TABLE category SCHEMALESS;
        DEFINE FIELD name ON category TYPE string;
        DEFINE TABLE product SCHEMALESS;
        DEFINE FIELD name ON product TYPE string;
        DEFINE FIELD price ON product TYPE float;
        DEFINE FIELD qty ON product TYPE int;
        DEFINE FIELD category ON product TYPE record <category>;",
    )
    .await?;*/

    println!("Tables created successfully!");

    let categories = vec!["Fruits", "Meat", "Vegetables", "Dairy", "Nuts"];

    // Insert categories
    for category in categories {
        let query = format!("CREATE category:{} SET name = '{}';", category, category);
        db.query(query).await?;
    }

    println!("Category table created and populated successfully!");

    Ok(())
}
