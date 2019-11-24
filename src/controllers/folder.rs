use super::ControllerError as Error;
use entities::models::{Folder, User};
use diesel::result;
use policies::Restricted;
use services::FolderService;

pub struct FolderController;

impl FolderController {
    pub fn index(user: User, parent_id: Option<i32>) -> Result<Vec<Folder>, Error> {
        let folder_service = resolve!(FolderService);

        if !user.can_index::<Folder>() {
            return Err(Error::Forbidden);
        }

        let mut folders = match folder_service.all(user.id()) {
            Ok(folders) => folders,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        match parent_id {
            Some(parent_id) => folders.retain(| folder | folder.id() == parent_id),
            None => folders.retain(| folder | folder.parent_id().is_none()),
        };

        Ok(folders)
    }

    pub fn show(user: User, folder_id: i32) -> Result<Folder, Error> {
        let folder_service = resolve!(FolderService);

        let found: Folder = match folder_service.find(folder_id) {
            Ok(folder) => folder,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        match user.can_view(found.clone()) {
            true => Ok(found),
            false => Err(Error::Forbidden),
        }
    }

    pub fn create(user: User) -> Result<(), Error> {
        match user.can_create::<Folder>() {
            true => Ok(()),
            false => Err(Error::Forbidden),
        }
    }

    pub fn store(
        user: User,
        name: String,
        user_id: i32,
        parent_id: Option<i32>,
    ) -> Result<Folder, Error> {
        let folder_service = resolve!(FolderService);

        if !user.can_create::<Folder>() {
            return Err(Error::Forbidden);
        }

        match folder_service.create(name, user_id, parent_id) {
            Ok(folder) => Ok(folder),
            Err(_) => Err(Error::InternalServerError),
        }
    }

    pub fn edit(user: User, folder_id: i32) -> Result<Folder, Error> {
        let folder_service = resolve!(FolderService);

        let found: Folder = match folder_service.find(folder_id) {
            Ok(folder) => folder,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        match user.can_modify(found.clone()) {
            true => Ok(found),
            false => Err(Error::Forbidden),
        }
    }

    pub fn update(
        user: User,
        folder_id: i32,
        name: String,
        user_id: i32,
        parent_id: Option<i32>,
    ) -> Result<Folder, Error> {
        let folder_service = resolve!(FolderService);

        let found: Folder = match folder_service.find(folder_id) {
            Ok(folder) => folder,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_modify(found.clone()) {
            return Err(Error::Forbidden);
        }

        match folder_service.update(folder_id, name, user_id, parent_id) {
            Ok(folder) => Ok(folder),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }

    pub fn delete(user: User, folder_id: i32) -> Result<Folder, Error> {
        let folder_service = resolve!(FolderService);

        let found: Folder = match folder_service.find(folder_id) {
            Ok(folder) => folder,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_delete(found.clone()) {
            return Err(Error::Forbidden);
        }

        match folder_service.delete(folder_id) {
            Ok(folder) => Ok(folder),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }
}
