# Rust and MYSQL API template
This is a template for a Rust API that connects to a MYSQL database. It uses the [mysql crate](https://crates.io/crates/mysql) to connect to the database and [actix web](https://actix.rs/) to run the web server..

## Setup
1. Install Rust
2. Setup a Mysql database, for example [with docker](https://hub.docker.com/_/mysql/):
3. Copy the `.env.example` file to `.env` and fill in the correct values
4. Run the API with `cargo run`

## Adding routes
To add routes, add a new file in the `routes` folder.
The file should contain a function that takes a reference to the MySQL Pool object and returns a valid API response.
Add the route to the `routes/mod.rs` file. You can create a model for your data in the `models` folder.
See the pets example for more information. Finally, you need to register the route(s) in the `main.rs` file.
