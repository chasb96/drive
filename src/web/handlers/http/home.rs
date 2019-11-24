use entities::models::{File, Folder, User};
use rocket::get;
use rocket::http::Status;
use rocket::response::Responder;
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use web::guards::auth::Auth;
use web::state::State;

#[derive(Serialize)]
struct HomeContext {
    user: User,
    folder: Folder,
    parent: Folder,
    folders: Vec<Folder>,
    files: Vec<File>,
}

#[get("/")]
pub fn home(auth: Auth, state: State) -> impl Responder<'static> {
    let file_controller = resolve!(FileController);
    let user = auth.to_owned().user();

    let folder = match <resolve!(FolderController)>::index(user.clone(), None) {
        Ok(folders) => match folders.first() {
            Some(root) => root.to_owned(),
            None => return Err(Status::InternalServerError),
        },
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

    let folders = match <resolve!(FolderController)>::index(user.clone(), Some(folder.id())) {
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

    let files = match file_controller.index(user.clone(), folder.id()) {
        Ok(files) => files,
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

    let context = state.into_context(HomeContext {
        user,
        // Root is used twice in two different contexts
        parent: folder.clone(),
        folder,
        folders,
        files,
    });

    Ok(Template::render("home", &context))
}
