// External
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

// Dependencies

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
//use diesel_migrations::run_pending_migrations_in_directory;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Modules

mod errors;
mod handlers;
mod models;
mod schema;

embed_migrations!("migrations");

// Helper functions

// Main function

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    embedded_migrations::run(&pool.get().expect("could not get db connection"))
        .expect("could not run migrations");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
