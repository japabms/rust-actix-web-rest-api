use diesel::{
    prelude::*,
    sql_query, pg::Pg, 
};
use diesel_migrations::{self, embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::{env, error::Error};

pub const MIGRATION: EmbeddedMigrations = embed_migrations!("../migrations");

pub fn run_migrations(conn: &mut impl MigrationHarness<Pg>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

    conn.run_pending_migrations(MIGRATION)?;
    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Erro ao conectar a {database_url}"))
}

//testing some things
pub fn reset_serial(table_name: &str, column_name: &str, mut conn: PgConnection) -> QueryResult<()>{
    let query = format!("ALTER SEQUENCE {} RESTART WITH 1", format!("{}_{}_seq", table_name, column_name));
    sql_query(query).execute(&mut conn)?;
    Ok(())
}


