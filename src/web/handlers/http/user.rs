use entities::models::User;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{Redirect, Responder};
use rocket::FromForm;
use rocket::{get, post};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use web::guards::auth::Auth;
use web::state::State;

#[derive(Serialize)]
pub struct IndexContext {
    user: User,
    users: Vec<User>,
}

#[get("/users")]
pub fn index(auth: Auth, state: State) -> impl Responder<'static> {
    let user = auth.clone().user();

    let users = match <resolve!(UserController)>::index(user.clone()) {
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

    let context = state.into_context(IndexContext { user, users });

    Ok(Template::render("user/index", &context))
}

#[derive(Serialize)]
pub struct ShowContext {
    user: User,
    show: User,
}

#[get("/users/<user_id>")]
pub fn show(auth: Auth, state: State, user_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    let show = match <resolve!(UserController)>::show(user.clone(), user_id) {
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

    let context = state.into_context(ShowContext { user, show });

    Ok(Template::render("user/show", &context))
}

#[derive(Serialize)]
pub struct CreateContext {
    user: User,
}

#[get("/users/create")]
pub fn create(auth: Auth, state: State) -> impl Responder<'static> {
    let user = auth.clone().user();

    if let Err(e) = <resolve!(UserController)>::create(user.clone()) {
        return Err(Status::from(e));
    }

    let context = state.into_context(CreateContext { user });

    Ok(Template::render("user/create", &context))
}

#[derive(FromForm)]
pub struct StorePayload {
    name: String,
    email: String,
    password: String,
}

#[post("/users", data = "<payload>")]
pub fn store(auth: Auth, payload: Form<StorePayload>) -> impl Responder<'static> {
    let user = auth.clone().user();

    let user = match <resolve!(UserController)>::store(
        user.clone(),
        payload.name.clone(),
        payload.email.clone(),
        "guest".to_string(),
        payload.password.clone(),
    ) {
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

    Ok(Redirect::to(format!("/users/{}", user.id())))
}

#[derive(Serialize)]
pub struct EditContext {
    user: User,
    edit: User,
}

#[get("/users/<user_id>/edit")]
pub fn edit(auth: Auth, state: State, user_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    let edit = match <resolve!(UserController)>::edit(user.clone(), user_id) {
        Ok(edit) => edit,
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

    let context = state.into_context(EditContext { user, edit });

    Ok(Template::render("user/edit", &context))
}

#[derive(FromForm)]
pub struct UpdatePayload {
    name: String,
    email: String,
    password: String,
}

#[post("/users/<user_id>", data = "<payload>")]
pub fn update(auth: Auth, user_id: i32, payload: Form<UpdatePayload>) -> impl Responder<'static> {
    let user = auth.clone().user();

    if !user.password_check(&payload.password) {
        return Err(Status::Forbidden);
    }

    match <resolve!(UserController)>::update(
        user.clone(),
        user_id,
        payload.name.clone(),
        payload.email.clone(),
        user.role().to_string(),
    ) {
        Ok(user) => Ok(Redirect::to(format!("/users/{}", user.id()))),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/users/<user_id>/delete")]
pub fn delete(auth: Auth, user_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    match <resolve!(UserController)>::delete(user, user_id) {
        Ok(_) => Ok(Redirect::to("/users")),
        Err(e) => Err(Status::from(e)),
    }
}

#[derive(FromForm)]
pub struct UpdatePasswordPayload {
    old_password: String,
    password: String,
}

#[post("/users/<user_id>/password", data = "<payload>")]
pub fn update_password(
    auth: Auth,
    user_id: i32,
    payload: Form<UpdatePasswordPayload>,
) -> impl Responder<'static> {
    let user = auth.clone().user();

    if !user.password_check(&payload.old_password) {
        return Err(Status::Forbidden);
    }

    match <resolve!(UserController)>::update_password(user, user_id, payload.password.to_string()) {
        Ok(user) => Ok(Redirect::to(format!("/users/{}", user.id()))),
        Err(e) => Err(Status::from(e)),
    }
}
