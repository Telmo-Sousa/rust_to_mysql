use std::env;
use mysql::{Pool, PooledConn};
use mysql::prelude::*;

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

fn establish_connection() -> Pool {
    let db_username = env::var("DB_USERNAME").expect("DB_USERNAME not set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
    let db_host = env::var("DB_HOST").expect("DB_HOST not set");
    let db_name = env::var("DB_NAME").expect("DB_NAME not set");
    let connection_url = format!("mysql://{}:{}@{}/{}", db_username, db_password, db_host, db_name);
    Pool::new(&connection_url).expect("Failed to create connection pool")
}

fn create_user(conn: &mut PooledConn, name: &str) -> Result<User, mysql::Error> {
    conn.exec_drop("INSERT INTO users (name) VALUES (?)", (name,))?;
    let id = conn.last_insert_id();
    Ok(User { id, name: name.to_string() })
}

fn read_users(conn: &mut PooledConn) -> Result<Vec<User>, mysql::Error> {
    conn.query_map("SELECT id, name FROM users", |(id, name)| User { id, name })
}

fn update_user(conn: &mut PooledConn, user_id: i32, new_name: &str) -> Result<(), mysql::Error> {
    conn.exec_drop("UPDATE users SET name = ? WHERE id = ?", (new_name, user_id))?;
    Ok(())
}

fn delete_user(conn: &mut PooledConn, user_id: i32) -> Result<(), mysql::Error> {
    conn.exec_drop("DELETE FROM users WHERE id = ?", (user_id,))?;
    Ok(())
}

fn main() {
    let mut conn = establish_connection().get_conn().expect("Failed to get connection from pool");

    let new_user = create_user(&mut conn, "Homem do Bussaco").expect("Error creating user");
    println!("User created: {:?}", new_user);

    match read_users(&mut conn) {
        Ok(users) => {
            for user in users {
                println!("User ID: {}, Name: {}", user.id, user.name);
            }
        }
        Err(err) => eprintln!("Error retrieving user information: {:?}", err),
    }

    update_user(&mut conn, new_user.id as i32, "Renato Alexandre").expect("Error updating user");

    println!("User updated");

    match read_users(&mut conn) {
        Ok(users) => {
            for user in users {
                println!("User ID: {}, Name: {}", user.id, user.name);
            }
        }
        Err(err) => eprintln!("Error retrieving user information: {:?}", err),
    }

    delete_user(&mut conn, new_user.id as i32).expect("Error deleting user");
    println!("User deleted");
}
