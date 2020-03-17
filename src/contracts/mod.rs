#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::contracts;


pub mod handlers;
pub mod router;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "contracts"]
pub struct Contract {
    pub id: i32,
    pub vendor: String,
    pub starts_on: String,
    pub ends_on: String,
    pub price: f64

}
