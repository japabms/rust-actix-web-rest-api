use diesel::{
    pg::Pg,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use diesel_migrations::{self, embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::{env, error::Error};

pub type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub const MIGRATION: EmbeddedMigrations = embed_migrations!();

//não sei se isso funciona
pub fn run_migrations(
    conn: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATION)?;
    Ok(())
}

pub fn get_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Erro ao criar pool")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Erro ao conectar a {database_url}"))
}

