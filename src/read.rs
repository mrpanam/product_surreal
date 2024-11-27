use crate::model::{Category, Product};
use std::collections::HashSet;
use std::fs;
pub fn read_json() -> (Vec<Product>, HashSet<Category>) {
    let data = fs::read_to_string("src/products.json").expect("Unable to read file");

    let product_list: Vec<Product> =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    let mut unique_categories: HashSet<Category> = HashSet::new();

    for product in &product_list {
        unique_categories.insert(product.category.clone());
        println!("product: {:#?}", product);
    }

    println!("Unique categories:");
    for category in unique_categories.clone() {
        println!("{:?}", &category.name); // Access the name field of the category);
    }
    println!("json read complete");
    (product_list, unique_categories)
}
