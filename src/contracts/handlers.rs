use crate::connection::DbConn;
use diesel::result::Error;
use crate::contracts;
use crate::contracts::Contract;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(conn: DbConn) -> Result<Json<Vec<Contract>>, Status> {
    contracts::repository::all(&conn)
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
pub fn create(contract: Json<Contract>, conn: DbConn) -> Result<Status, Status> {
    match contracts::repository::insert(contract.into_inner(), &conn){
        Ok(_) => Ok(Status::Created),
        Err(error) => Err(error_status(error))
    }
}

#[put("/<id>", format = "application/json", data = "<contract>")]
pub fn update(id: i32, contract: Json<Contract>, conn: DbConn) -> Result<Status, Status> {
    match contracts::repository::update(id, contract.into_inner(), &conn){
        Ok(_) => Ok(Status::NoContent),
        Err(error) => Err(error_status(error))
    }
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
