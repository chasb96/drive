use controllers::folder::UpdateRequest;
use controllers::folder::StoreRequest;
use entities::presentation::ToJson;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{get, post};
use rocket_contrib::json::Json;
use serde_derive::Deserialize;
use web::guards::auth::Auth;
use controllers::folder::FolderController;

#[get("/folders?<parent_id>")]
pub fn index(auth: Auth, parent_id: Option<i32>) -> impl Responder<'static> {
    let folder_controller = resolve!(FolderController);
    let user = auth.user();

    let folders = match folder_controller.index(user.clone(), parent_id) {
        Ok(folders) => folders,
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

    Ok(folders.to_json())
}

#[get("/folders/<folder_id>")]
pub fn show(auth: Auth, folder_id: i32) -> impl Responder<'static> {
    let folder_controller = resolve!(FolderController);
    let user = auth.user();

    let folder = match folder_controller.show(user.clone(), folder_id) {
        Ok(folder) => folder,
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

    Ok(folder.to_json())
}

#[derive(Deserialize)]
pub struct StorePayload {
    name: String,
    parent_id: Option<i32>,
}

#[post("/folders", data = "<payload>")]
pub fn store(auth: Auth, payload: Json<StorePayload>) -> impl Responder<'static> {
    let folder_controller = resolve!(FolderController);
    let user = auth.user();

    let store_request = StoreRequest {
        name: payload.name.to_owned(),
        user_id: user.id(),
        parent_id: payload.parent_id
    };

    match folder_controller.store(user.clone(), store_request) {
        Ok(_) => Ok(Status::Created),
        Err(e) => {
            log!(
                e.level(),
                "Request from user \"{}\" returned \"{}\"",
                user.id(),
                e
            );

            Err(Status::from(e))
        }
    }
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    name: String,
    parent_id: Option<i32>,
}

#[post("/folders/<folder_id>", data = "<payload>")]
pub fn update(auth: Auth, folder_id: i32, payload: Json<UpdatePayload>) -> impl Responder<'static> {
    let folder_controller = resolve!(FolderController);
    let user = auth.user();

    let update_request = UpdateRequest {
        folder_id,
        name: payload.name.to_owned(),
        user_id: user.id(),
        parent_id: payload.parent_id
    };

    match folder_controller.update(user, update_request) {
        Ok(_) => Ok(Status::Ok),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/folders/<folder_id>/delete")]
pub fn delete(auth: Auth, folder_id: i32) -> impl Responder<'static> {
    let folder_controller = resolve!(FolderController);
    let user = auth.user();

    match folder_controller.delete(user, folder_id) {
        Ok(_) => Ok(Status::Ok),
        Err(e) => Err(Status::from(e)),
    }
}
