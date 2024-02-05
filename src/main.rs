use std::env;
use mysql::{Pool, PooledConn};
use mysql::prelude::*;

struct User {
    id: i32,
    name: String,
}

fn main() {

    let db_username = env::var("DB_USERNAME").expect("DB_USERNAME not set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
    let db_host = env::var("DB_HOST").expect("DB_HOST not set");
    let db_name = env::var("DB_NAME").expect("DB_NAME not set");

    let connection_url = format!("mysql://{}:{}@{}/{}", db_username, db_password, db_host, db_name);
    let pool = Pool::new(&connection_url).expect("Failed to create connection pool");

    let mut conn: PooledConn = pool.get_conn().expect("Failed to get connection from pool");

    match conn.query_map("SELECT id, name FROM users", |(id, name)| User { id, name }) {
        Ok(users) => {

            for user in users {
                println!("User ID: {}, Name: {}", user.id, user.name);
            }
        }
        Err(err) => {
            eprintln!("Error retrieving user information: {:?}", err);
        }
    }
}

