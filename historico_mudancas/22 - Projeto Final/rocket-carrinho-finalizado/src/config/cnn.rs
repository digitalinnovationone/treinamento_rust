extern crate rusqlite;
extern crate dotenv;

use rusqlite::Connection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Connection {
    dotenv().ok();

    let database_path = env::var("DATABASE_PATH")
        .expect("DATABASE_PATH must be set");
    
    Connection::open(database_path)
        .expect("Error connecting to the database")
}
