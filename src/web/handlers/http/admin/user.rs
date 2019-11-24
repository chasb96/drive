use entities::models::User;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{Redirect, Responder};
use rocket::FromForm;
use rocket::{get, post};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use web::guards::admin::Admin;
use web::state::State;
use web::success::Success;

#[derive(Serialize)]
pub struct IndexContext {
    user: User,
    users: Vec<User>,
}

#[get("/admin/users")]
pub fn index(admin: Admin, state: State) -> impl Responder<'static> {
    let user = admin.clone().user();

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

    Ok(Template::render("admin/user/index", &context))
}

#[derive(Serialize)]
pub struct ShowContext {
    user: User,
    show: User,
}

#[get("/admin/users/<user_id>")]
pub fn show(admin: Admin, state: State, user_id: i32) -> impl Responder<'static> {
    let user = admin.clone().user();

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

    Ok(Template::render("admin/user/show", &context))
}

#[derive(Serialize)]
pub struct CreateContext {
    user: User,
}

#[get("/admin/users/create")]
pub fn create(admin: Admin, state: State) -> impl Responder<'static> {
    let user = admin.clone().user();

    if let Err(e) = <resolve!(UserController)>::create(user.clone()) {
        return Err(Status::from(e));
    }

    let context = state.into_context(CreateContext { user });

    Ok(Template::render("admin/user/create", &context))
}

#[derive(FromForm)]
pub struct StorePayload {
    name: String,
    email: String,
    role: String,
    password: String,
}

#[post("/admin/users", data = "<payload>")]
pub fn store(
    admin: Admin,
    mut state: State,
    payload: Form<StorePayload>,
) -> impl Responder<'static> {
    let user = admin.clone().user();

    let user = match <resolve!(UserController)>::store(
        user.clone(),
        payload.name.clone(),
        payload.email.clone(),
        payload.role.clone(),
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

    state.push_success(Success::new(format!(
        "User {} successfully created!",
        user.email(),
    )));

    Ok(Redirect::to(format!("/admin/users/{}", user.id())))
}

#[derive(Serialize)]
pub struct EditContext {
    user: User,
    edit: User,
}

#[get("/admin/users/<user_id>/edit")]
pub fn edit(admin: Admin, state: State, user_id: i32) -> impl Responder<'static> {
    let user = admin.clone().user();

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

    Ok(Template::render("admin/user/edit", &context))
}

#[derive(FromForm)]
pub struct UpdatePayload {
    name: String,
    email: String,
    role: String,
}

#[post("/admin/users/<user_id>", data = "<payload>")]
pub fn update(admin: Admin, user_id: i32, payload: Form<UpdatePayload>) -> impl Responder<'static> {
    let user = admin.clone().user();

    match <resolve!(UserController)>::update(
        user,
        user_id,
        payload.name.clone(),
        payload.email.clone(),
        payload.role.clone(),
    ) {
        Ok(user) => Ok(Redirect::to(format!("/admin/users/{}", user.id()))),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/admin/users/<user_id>/delete")]
pub fn delete(admin: Admin, user_id: i32) -> impl Responder<'static> {
    let user = admin.clone().user();

    if user.id() == user_id {
        return Err(Status::Forbidden);
    }

    match <resolve!(UserController)>::delete(user, user_id) {
        Ok(_) => Ok(Redirect::to("/admin/users")),
        Err(e) => Err(Status::from(e)),
    }
}
