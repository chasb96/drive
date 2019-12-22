#![feature(proc_macro_hygiene, decl_macro, specialization)]
#![feature(optin_builtin_traits)]
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;
extern crate rand;
extern crate time;
#[macro_use]
extern crate lazy_static;
extern crate diesel_derive_enum;
extern crate frank_jwt;
extern crate postgres;
extern crate r2d2_postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_multipart_form_data;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate s3;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

#[cfg(test)]
#[macro_use]
extern crate fake;

#[macro_use]
mod di;

#[macro_use]
mod logging;

#[cfg(test)]
#[macro_use]
mod test;


mod auth;
mod controllers;
mod entities;
mod env;
mod policies;
mod schema;
mod storage_drivers;
mod web;
mod services;

fn main() {
    // Load .env file
    dotenv::dotenv().expect("Missing .env file");

    seed();

    web::boot()
}

use crate::entities::models::User;
use crate::entities::diesel::pool::DbPool;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use crate::schema::*;
use crate::services::UserService;
use crate::services::user::CreateRequest;

fn seed() {
    let user_service = resolve!(UserService);

    match User::all()
        .filter(users::role.eq("admin"))
        .first::<User>(&DbPool::connection())
    {
        Ok(_) => {}
        Err(_) => {
            let request = CreateRequest {
                name: "Temp Admin".to_string(),
                email: "temp@temp.com".to_string(),
                role: "admin".to_string(),
                password: "password".to_string(),
            };

            user_service.create(request).unwrap();
        }
    };
}
