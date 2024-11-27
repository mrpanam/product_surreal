mod connect;
mod helper;
mod init;
mod model;
mod read;
mod select;

#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    println!("Hello, world!");

    let db = connect::connect().await?;

    init::init_db(&db).await?;

    //let (products, categories) = read::read_json();

    //helper::create_categories(categories, &db).await?;

    //select::select_category(&db).await?;

    helper::create_product(&db).await?;

    //helper::create_products(products, &db).await?;

    Ok(())
}
