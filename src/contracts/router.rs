use crate::contracts;
use crate::connection;
use rocket;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/contracts",
            routes![contracts::handlers::index,
                contracts::handlers::show,
                contracts::handlers::create,
                contracts::handlers::update,
                contracts::handlers::delete]
            ).launch();
}
