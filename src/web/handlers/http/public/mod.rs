use db::builders::{Builder, UserBuilder};
use db::models::{File, User};
use env::Env;
use rocket::get;
use rocket::http::Status;
use rocket::response::{Responder, Stream};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use web::state::State;

#[derive(Serialize)]
pub struct FileContext {
    user: User,
    file: File,
}

#[get("/public/<file_id>")]
pub fn file(file_id: i32, state: State) -> impl Responder<'static> {
    let file_controller = resolve!(FileController);
    let user = UserBuilder::new()
        .with_name("Guest".to_string())
        .with_role("guest".to_string())
        .build();

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

    let context = state.into_context(FileContext { user, file });

    Ok(Template::render("public/file", &context))
}

#[get("/public/<file_id>/download")]
pub fn download(file_id: i32) -> impl Responder<'static> {
    let file_controller = resolve!(FileController);
    let user = UserBuilder::new()
        .with_name("Guest".to_string())
        .with_role("guest".to_string())
        .build();

    let stream = match file_controller.contents(user.clone(), file_id) {
        Ok(contents) => contents,
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
