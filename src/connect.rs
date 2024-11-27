use surrealdb::engine::any;
use surrealdb::opt::auth::Root;

use surrealdb::{Error, Surreal};

pub async fn connect() -> Result<Surreal<any::Any>, Error> {
    let db = any::connect("ws://localhost:8080/rpc").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("test").use_db("test").await?;
    print!("Connected to database\n");
    Ok(db)
}
