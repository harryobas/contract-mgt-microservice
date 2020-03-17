use connection::DbConn;
use diesel::result::Error;
use std::env;
use contracts;
use contracts::Contract;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(conn: DbConn) -> Result<Json<Vec<Contract>>, Status> {
    contracts::repository.all(&conn)
        .map(|contracts| Json(contracts))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn show(id: i32, conn: DbConn) -> Result<Json<Contract>, Status> {
    contracts::repository::get(id, &conn)
        .map(|contract| Json(contract))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<contract>")]
pub fn create(contract: Json<Contract>, conn: DbConn) -> Result<status::Created<Json<Contract>>, Status> {
    contracts::repository::insert(contract.into_inner(), &conn)
        .map(|contract| contract_created(contract))
        .map_err(|error| error_status(error))
}

fn contract_created(contract: Contract) -> status::Created<Json<Contract>> {
    status::Created(
        format!("{host}:{port}/people/{id}", host = host(), port = port(), id = contract.id).to_string(),
        Some(Json(contract)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<contract>")]
pub fn update(id: i32, contract: Json<Contract>, conn: DbConn) -> Result<Json<Contract>, Status> {
    contracts::repository::update(id, contract.into_inner(), &conn)
        .map(|contract| Json(contract))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Result<Status, Status> {
    match contracts::repository::get(id, &conn){
        Ok(_) => contracts::repository::delete(id, &conn)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
