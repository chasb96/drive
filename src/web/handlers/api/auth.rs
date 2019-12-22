use crate::auth::authenticate::Authenticate;
use crate::auth::basic::Credentials;
use crate::auth::bearer::Bearer;
use crate::auth::Auth;
use crate::entities::models::User;
use rocket::http::Status;
use rocket::post;
use rocket::response::Responder;
use rocket_contrib::json::Json;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

#[post("/login", data = "<payload>")]
pub fn login(payload: Json<LoginPayload>) -> impl Responder<'static> {
    let credentials = Credentials::new(payload.email.clone(), payload.password.clone());

    let user: User = match Auth::Basic(credentials).verify() {
        Ok(user) => user,
        Err(_) => return Err(Status::Unauthorized),
    };

    let token = match user.encode() {
        Ok(token) => token,
        Err(e) => {
          log!("error", "500 Internal Server Error: {}", e);
          return Err(Status::InternalServerError);
      }
    };

    Ok(token)
}
