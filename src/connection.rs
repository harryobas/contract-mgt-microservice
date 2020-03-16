use diesel::sqlite::SqliteConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use dotenv::dotenv;
use std::env;
use std::ops::Deref;

type pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url());
    pool.new(manager).expect("db pool")

}

fn database_url() -> String {
    dotenv.ok();
    env::var("DATABASE_URL")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Request::Outcome<DbConn, self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &self.Target{
        &self.0
    }
}
