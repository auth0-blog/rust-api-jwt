// src/handlers.rs

// Dependencies

use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{error, web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

/// Handler for GET /users
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    web::block(move || get_all_users(db))
        .await
        .unwrap()
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(error::ErrorInternalServerError)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

/// Handler for GET /users/{id}
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    web::block(move || db_get_user_by_id(db, user_id.into_inner()))
        .await
        .unwrap()
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|err| error::ErrorInternalServerError(err))
}

/// Handler for POST /users
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    web::block(move || add_single_user(db, item))
        .await
        .unwrap()
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|err| error::ErrorInternalServerError(err))
}

/// Handler for DELETE /users/{id}
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    web::block(move || delete_single_user(db, user_id.into_inner()))
        .await
        .unwrap()
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|err| error::ErrorInternalServerError(err))
}

fn db_get_user_by_id(
    pool: web::Data<Pool>,
    user_id: String,
) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

fn add_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<String, diesel::result::Error> {
    let conn = db.get().unwrap();
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    let new_user = NewUser {
        id: &uuid,
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    insert_into(users).values(&new_user).execute(&conn)?;

    Ok(uuid)
}

fn delete_single_user(
    db: web::Data<Pool>,
    user_id: String,
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    delete(users.find(user_id)).execute(&conn)
}
