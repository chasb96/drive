use controllers::user::StoreRequest;
use controllers::user::UpdateRequest;
use entities::presentation::ToJson;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{get, post};
use rocket_contrib::json::Json;
use serde_derive::Deserialize;
use web::guards::auth::Auth;
use controllers::UserController;

#[get("/users")]
pub fn index(auth: Auth) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    let users = match user_controller.index(user.clone()) {
        Ok(users) => users,
        Err(e) => {
            log!(
                e.level(),
                "Request from user \"{}\" returned \"{}\"",
                user.id(),
                e
            );
            return Err(Status::from(e));
        }
    };

    Ok(users.to_json())
}

#[get("/users/<user_id>")]
pub fn show(auth: Auth, user_id: i32) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    let show = match user_controller.show(user.clone(), user_id) {
        Ok(user) => user,
        Err(e) => {
            log!(
                e.level(),
                "Request from user \"{}\" returned \"{}\"",
                user.id(),
                e
            );
            return Err(Status::from(e));
        }
    };

    Ok(show.to_json())
}

#[derive(Deserialize)]
pub struct StorePayload {
    name: String,
    email: String,
    password: String,
}

#[post("/users", data = "<payload>")]
pub fn store(auth: Auth, payload: Json<StorePayload>) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    let store_request = StoreRequest {
        name: payload.name.clone(),
        email: payload.email.clone(),
        role: "guest".to_string(),
        password: payload.password.clone()
    };

    match user_controller.store(user.clone(), store_request) {
        Ok(_) => {}
        Err(e) => {
            log!(
                e.level(),
                "Request from user \"{}\" returned \"{}\"",
                user.id(),
                e
            );
            return Err(Status::from(e));
        }
    };

    Ok(Status::Created)
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    name: String,
    email: String,
    password: String,
}

#[post("/users/<user_id>", data = "<payload>")]
pub fn update(auth: Auth, user_id: i32, payload: Json<UpdatePayload>) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    if !user.password_check(&payload.password) {
        return Err(Status::Forbidden);
    }

    let update_request = UpdateRequest {
        user_id,
        name: payload.name.clone(),
        email: payload.email.clone(),
        role: user.role().to_string()
    };

    match user_controller.update(user, update_request) {
        Ok(_) => Ok(Status::Ok),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/users/<user_id>/delete")]
pub fn delete(auth: Auth, user_id: i32) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    match user_controller.delete(user, user_id) {
        Ok(_) => Ok(Status::Ok),
        Err(e) => Err(Status::from(e)),
    }
}
