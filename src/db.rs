use diesel::{prelude::*, r2d2::{self, ConnectionManager}};
use dotenv::dotenv;
use std::env;

pub type Connection = PgConnection;

pub type Pool =  r2d2::Pool<ConnectionManager<Connection>>;

pub fn establish_connection() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Erro ao criar a pool");
    pool
}
