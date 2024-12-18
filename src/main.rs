mod connect;
mod helper;
mod init;
mod model;
mod read;

#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    println!("Hello, world!");

    let db = connect::connect().await?;

    let (products, categories) = read::read_json();

    helper::create_categories(categories, &db).await?;

    helper::create_products(products, &db).await?;

    Ok(())
}
