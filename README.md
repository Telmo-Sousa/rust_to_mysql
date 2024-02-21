# rust_to_mysql

This project is a simple command-line application written in Rust that demonstrates how to perform CRUD (Create, Read, Update, Delete) operations on a MySQL database. The application defines a User struct with id and name fields, and provides functions to create, read, update, and delete user records in the MySQL database.

The establish_connection function creates a connection pool to the MySQL database using the mysql crate, and the create_user, read_users, update_user, and delete_user functions perform CRUD operations on the users table in the database.

The main function demonstrates how to use these functions to create a new user, read all users, update a user, and delete a user from the database. The application uses environment variables to store the database connection details, and provides error handling for various scenarios.

Overall, this project is a good example of how to use Rust to interact with a MySQL database and perform CRUD operations on a simple data model.

## How to Run

### Prerequisites
Make sure you have Rust and a MySQL server installed on your machine.
<br>
The table in your MySQL database should have the id set to auto increment.

### Set Environment Variables
Before running the project, set the following environment variables with your MySQL database credentials:

```bash
export DB_USERNAME=your_username
export DB_PASSWORD=your_password
export DB_HOST=your_host
export DB_NAME=your_database
```

### Run the Project
To run the project, execute the following command:

```bash
cargo run
```

## Example

![Image](/image.png)

This is a brief example of my project.
