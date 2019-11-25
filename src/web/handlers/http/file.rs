use controllers::file::CreateRequest;
use controllers::file::UpdateRequest;
use entities::models::{File, Folder, User};
use env::Env;
use rocket::data::Data;
use rocket::http::{ContentType, Status};
use rocket::request::Form;
use rocket::response::Stream;
use rocket::response::{Redirect, Responder};
use rocket::FromForm;
use rocket::{get, post};
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::{
    FileField, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use serde_derive::Serialize;
use std::fs;
use web::guards::auth::Auth;
use web::state::State;
use web::success::Success;
use crate::controllers::file::FileController;

#[get("/folders/<folder_id>/files")]
pub fn index(_auth: Auth, folder_id: i32) -> impl Responder<'static> {
    Redirect::to(format!("/folders/{}", folder_id))
}

#[derive(Serialize)]
pub struct ShowContext {
    user: User,
    folder: Folder,
    file: File,
}

#[get("/folders/<_folder_id>/files/<file_id>", rank = 2)]
pub fn show(auth: Auth, state: State, _folder_id: i32, file_id: i32) -> impl Responder<'static> {
    let file_controller = resolve!(FileController);
    let user = auth.user();

    let file = match file_controller.show(user.clone(), file_id) {
        Ok(file) => file,
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

    let folder = match <resolve!(FolderController)>::show(user.clone(), file.folder_id()) {
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

    let context = state.into_context(ShowContext { user, folder, file });

    Ok(Template::render("file/show", &context))
}

#[derive(Serialize)]
pub struct CreateContext {
    user: User,
    folder: Folder,
}

#[get("/folders/<folder_id>/files/create", rank = 1)]
pub fn create(auth: Auth, state: State, folder_id: i32) -> impl Responder<'static> {
    let file_controller = resolve!(FileController);
    let user = auth.user();

    if let Err(e) = file_controller.create(user.clone()) {
        return Err(Status::from(e));
    }

    let folder = match <resolve!(FolderController)>::show(user.clone(), folder_id) {
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

    let context = state.into_context(CreateContext { user, folder });

    Ok(Template::render("file/create", &context))
}

#[post("/folders/<folder_id>/files", data = "<payload>")]
pub fn store(
    auth: Auth,
    mut state: State,
    folder_id: i32,
    payload: Data,
    content_type: &ContentType,
) -> impl Responder<'static> {
    let file_controller = resolve!(FileController);
    let mut options = MultipartFormDataOptions::new();
    options
        .allowed_fields
        .push(MultipartFormDataField::file("file").size_limit(10_000_000_000));

    let multipart_form_data = MultipartFormData::parse(content_type, payload, options).unwrap();

    let file = multipart_form_data.files.get("file");
    let mut name = String::new();

    let file = match file {
        Some(FileField::Single(file)) => {
            let res = match fs::File::open(file.path.clone()) {
                Ok(res) => res,
                Err(_) => return Err(Status::BadRequest),
            };

            if let Some(file_name) = &file.file_name {
                name = file_name.to_string();
            }

            res
        }
        _ => return Err(Status::BadRequest),
    };

    let user = auth.user();

    let mut parts = name.splitn(2, '.');

    let store_request = CreateRequest {
        name: parts.nth(0).unwrap_or("").to_string(),
        extension: parts.nth(0).unwrap_or("").to_string(),
        user_id: user.id(),
        folder_id,
        public: false,
        input: file,
    };

    let stored = match file_controller.store(user.clone(), store_request) {
        Ok(file) => file,
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
        "File {}.{} successfully uploaded!",
        stored.name(),
        stored.extension()
    )));

    Ok(Redirect::to(format!(
        "/folders/{}/files/{}",
        folder_id,
        stored.id()
    )))
}

#[derive(Serialize)]
pub struct EditContext {
    user: User,
    folder: Folder,
    file: File,
}

#[get("/folders/<_folder_id>/files/<file_id>/edit")]
pub fn edit(auth: Auth, state: State, _folder_id: i32, file_id: i32) -> impl Responder<'static> {
    let file_controller = resolve!(FileController);
    let user = auth.user();

    let file = match file_controller.edit(user.clone(), file_id) {
        Ok(file) => file,
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

    let folder = match <resolve!(FolderController)>::edit(user.clone(), file.folder_id()) {
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

    let context = state.into_context(EditContext { user, folder, file });

    Ok(Template::render("file/edit", &context))
}

#[derive(FromForm)]
pub struct UpdatePayload {
    name: String,
    extension: String,
    public: bool,
    folder_id: i32,
}

#[post("/folders/<_folder_id>/files/<file_id>", data = "<payload>")]
pub fn update(
    auth: Auth,
    _folder_id: i32,
    file_id: i32,
    payload: Form<UpdatePayload>,
) -> impl Responder<'static> {
    let file_controller = resolve!(FileController);
    let user = auth.user();

    let request = UpdateRequest {
        file_id,
        name: payload.name.to_owned(),
        extension: payload.extension.to_owned(),
        public: payload.public,
        folder_id: payload.folder_id
    };

    match file_controller.update(user, request) {
        Ok(file) => Ok(Redirect::to(format!(
            "/folders/{}/files/{}",
            file.folder_id(),
            file.id()
        ))),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/folders/<_folder_id>/files/<file_id>/delete")]
pub fn delete(auth: Auth, _folder_id: i32, file_id: i32) -> impl Responder<'static> {
    let file_controller = resolve!(FileController);
    let user = auth.user();

    match file_controller.delete(user, file_id) {
        Ok(file) => Ok(Redirect::to(format!("/folders/{}", file.folder_id()))),
        Err(e) => Err(Status::from(e)),
    }
}

#[get("/folders/<_folder_id>/files/<file_id>/<_name>", rank = 1)]
pub fn download(
    auth: Auth,
    _folder_id: i32,
    file_id: i32,
    _name: String,
) -> impl Responder<'static> {
    let file_controller = resolve!(FileController);
    let user: User = auth.user();

    let stream = match file_controller.contents(user.clone(), file_id) {
        Ok(stream) => stream,
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

    let response = Stream::chunked(stream, Env::chunk_size());

    Ok(response)
}
