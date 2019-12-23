use rocket::{catchers, routes};

mod context;
mod error;
mod fairings;
mod guards;
mod handlers;
mod state;
mod success;

use self::handlers::*;
use crate::controllers::error::ControllerError;
use rocket::http::Status;
use rocket_contrib::templates::Template;

pub fn boot() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(fairings::log::LogFairing)
        .mount(
            "/",
            routes![
                http::resources,
                http::robots,
                http::auth::login,
                http::auth::authenticate,
                http::auth::logout,
                http::home::home,
                http::folder::index,
                http::folder::show,
                http::folder::create,
                http::folder::store,
                http::folder::edit,
                http::folder::update,
                http::folder::delete,
                http::file::index,
                http::file::show,
                http::file::create,
                http::file::store,
                http::file::edit,
                http::file::update,
                http::file::delete,
                http::file::download,
                http::user::index,
                http::user::show,
                http::user::create,
                http::user::store,
                http::user::edit,
                http::user::update,
                http::user::delete,
                http::user::update_password,
                http::admin::home,
                http::admin::user::index,
                http::admin::user::show,
                http::admin::user::create,
                http::admin::user::store,
                http::admin::user::edit,
                http::admin::user::update,
                http::admin::user::delete,
                http::public::file,
                http::public::download
            ],
        )
        .mount(
            "/api",
            routes![
                api::auth::login,
                api::user::index,
                api::user::show,
                api::user::store,
                api::user::update,
                api::user::delete,
                api::folder::index,
                api::folder::show,
                api::folder::store,
                api::folder::update,
                api::folder::delete,
            ],
        )
        .register(catchers![
            handlers::error::unauthorized,
            handlers::error::forbidden,
            handlers::error::not_found,
            handlers::error::internal_server_error,
            handlers::error::bad_request
        ])
        .launch();
}

impl From<ControllerError> for Status {
    fn from(error: ControllerError) -> Self {
        match error {
            ControllerError::Unauthorized => Status::Unauthorized,
            ControllerError::Forbidden => Status::Forbidden,
            ControllerError::NotFound => Status::NotFound,
            ControllerError::InternalServerError => Status::InternalServerError,
        }
    }
}
