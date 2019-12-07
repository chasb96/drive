use auth::authenticate::Authenticate;
use auth::basic::Credentials;
use auth::bearer::Token;
use auth::Auth;
use entities::models::User;
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::Form;
use rocket::response::{Redirect, Responder};
use rocket::FromForm;
use rocket::{get, post};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use std::convert::TryFrom;
use web::error::Error;
use web::state::State;

#[derive(Serialize)]
struct LoginContext {}

#[get("/login")]
pub fn login(state: State) -> impl Responder<'static> {
    let context = state.into_context(LoginContext {});

    Template::render("auth/login", &context)
}

#[derive(FromForm)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[post("/login", data = "<payload>")]
pub fn authenticate(mut state: State, payload: Form<LoginForm>) -> impl Responder<'static> {
    let credentials = Credentials::new(payload.email.clone(), payload.password.clone());

    let user: User = match Auth::Basic(credentials).verify() {
        Ok(user) => user,
        Err(_) => {
            state.push_error(Error::new(
                "Your email or password is incorrect.".to_string(),
            ));

            return Ok(Redirect::to("/login"));
        }
    };

    let token = match Token::try_from(user.clone()) {
        Ok(token) => token,
        Err(e) => {
            log!("error", "500 Internal Server Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    state
        .cookies()
        .add_private(Cookie::new("token", token.to_string()));

    log!("debug", "Got session from user {}", user.id());

    Ok(Redirect::to("/"))
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> impl Responder<'static> {
    cookies.remove_private(Cookie::named("token"));

    Redirect::to("/login")
}
