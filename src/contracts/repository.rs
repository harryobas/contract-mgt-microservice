#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::contracts;
use contracts::Contract;

pub fn all(conn: &SqliteConnection) -> QueryResult<Vec<Contract>> {
    contracts::table.load::<Contract>(&*conn)
}

pub fn get(id: i32, conn: &SqliteConnection) -> QueryResult<Contract> {
    contracts::table.find(id).get_result::<Contract>(conn)
}

pub fn insert(contract: Contract, conn: &SqliteConnection) -> QueryResult<Contract> {
    diesel::insert_into(contracts::table)
        .values(&InsertableContract::from_contract(contract))
        .get_result(conn)
}

pub fn update(id: i32, contract: Contract, conn: &SqliteConnection) -> QueryResult<Contract>{
    diesel::update(contracts::table.find(id))
        .set(&contract)
        .get_result(conn)
}

pub fn delete(id: i32 conn: &SqliteConnection) -> QueryResult<usize> {
    diesel::delete(contracts::table.find(id))
        .execute(conn)
}

#[derive(Insertable)]
#[table_name = "contracts"]
struct InsertableContract {
    vendor: String,
    starts_on: String,
    ends_on: String,
    price: f64
}

impl InsertableContract {
    fn from_contract(contract: Contract) -> Self {
        Self {
            vendor: contract.vendor,
            starts_on: contract.starts_on,
            ends_on: contract.ends_on,
            price: contract.price
        }
    }
}
