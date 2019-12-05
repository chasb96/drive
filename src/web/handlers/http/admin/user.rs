use controllers::user::UpdateRequest;
use controllers::user::StoreRequest;
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
use controllers::UserController;

#[derive(Serialize)]
pub struct IndexContext {
    user: User,
    users: Vec<User>,
}

#[get("/admin/users")]
pub fn index(admin: Admin, state: State) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = admin.user();

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
    let user_controller = resolve!(UserController);
    let user = admin.user();

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

    let context = state.into_context(ShowContext { user, show });

    Ok(Template::render("admin/user/show", &context))
}

#[derive(Serialize)]
pub struct CreateContext {
    user: User,
}

#[get("/admin/users/create")]
pub fn create(admin: Admin, state: State) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = admin.user();

    if let Err(e) = user_controller.create(user.clone()) {
        return Err(Status::from(e));
    }

    let context = state.into_context(CreateContext { user });

    Ok(Template::render("admin/user/create", &context))
}

#[derive(FromForm)]
pub struct StorePayload {
    name: String,
    email: String,
    _role: String,
    password: String,
}

#[post("/admin/users", data = "<payload>")]
pub fn store(
    admin: Admin,
    mut state: State,
    payload: Form<StorePayload>,
) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = admin.user();

    let store_request = StoreRequest {
        name: payload.name.clone(),
        email: payload.email.clone(),
        role: "guest".to_string(),
        password: payload.password.clone()
    };

    let user = match user_controller.store(user.clone(), store_request) {
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
    let user_controller = resolve!(UserController);
    let user = admin.user();

    let edit = match user_controller.edit(user.clone(), user_id) {
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
    _role: String,
}

#[post("/admin/users/<user_id>", data = "<payload>")]
pub fn update(admin: Admin, user_id: i32, payload: Form<UpdatePayload>) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = admin.user();

    let update_request = UpdateRequest {
        user_id,
        name: payload.name.clone(),
        email: payload.email.clone(),
        role: user.role().to_string()
    };

    match user_controller.update(user, update_request) {
        Ok(user) => Ok(Redirect::to(format!("/admin/users/{}", user.id()))),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/admin/users/<user_id>/delete")]
pub fn delete(admin: Admin, user_id: i32) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = admin.user();

    if user.id() == user_id {
        return Err(Status::Forbidden);
    }

    match user_controller.delete(user, user_id) {
        Ok(_) => Ok(Redirect::to("/admin/users")),
        Err(e) => Err(Status::from(e)),
    }
}
