use crate::controllers::error::ControllerError as Error;
use entities::models::{Folder, User};
use policies::Restricted;
use crate::services::folder::CreateRequest as ServiceCreateRequest;
use crate::services::folder::UpdateRequest as ServiceUpdateRequest;
use crate::services::error::ServiceError;
use crate::controllers::folder::FolderController;
use crate::services::FolderService;
use crate::controllers::folder::StoreRequest;
use crate::controllers::folder::UpdateRequest;

pub struct Controller<T: FolderService> {
    folder_service: T
}

impl<T: FolderService> Controller<T> {
    pub fn new(folder_service: T) -> Self {
        Self { folder_service }
    }
}

impl<T: FolderService> FolderController for Controller<T> {
    fn index(&self, user: User, parent_id: Option<i32>) -> Result<Vec<Folder>, Error> {
        if !user.can_index::<Folder>() {
            return Err(Error::Forbidden);
        }

        let mut folders = match self.folder_service.all(user.id()) {
            Ok(folders) => folders,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
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

    fn show(&self, user: User, folder_id: i32) -> Result<Folder, Error> {
        let found: Folder = match self.folder_service.find(folder_id) {
            Ok(folder) => folder,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if user.can_view(found.clone()) {
            Ok(found)
        } else {
            Err(Error::Forbidden)
        }
    }

    fn create(&self, user: User) -> Result<(), Error> {
        if user.can_create::<Folder>() {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    fn store(&self, user: User, request: StoreRequest) -> Result<Folder, Error> {
        if !user.can_create::<Folder>() {
            return Err(Error::Forbidden);
        }

        let service_create_request = ServiceCreateRequest {
            name: request.name,
            user_id: request.user_id,
            parent_id: request.parent_id,
        };

        match self.folder_service.create(service_create_request) {
            Ok(folder) => Ok(folder),
            Err(_) => Err(Error::InternalServerError),
        }
    }

    fn edit(&self, user: User, folder_id: i32) -> Result<Folder, Error> {
        let found: Folder = match self.folder_service.find(folder_id) {
            Ok(folder) => folder,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if user.can_modify(found.clone()) {
            Ok(found)
        } else {
            Err(Error::Forbidden)
        }
    }

    fn update(&self, user: User, request: UpdateRequest) -> Result<Folder, Error> {
        let found: Folder = match self.folder_service.find(request.folder_id) {
            Ok(folder) => folder,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_modify(found) {
            return Err(Error::Forbidden);
        }

        let service_update_request = ServiceUpdateRequest {
            id: request.folder_id,
            name: request.name,
            user_id: request.user_id,
            parent_id: request.parent_id,
        };

        match self.folder_service.update(service_update_request) {
            Ok(folder) => Ok(folder),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }

    fn delete(&self, user: User, folder_id: i32) -> Result<Folder, Error> {
        let found: Folder = match self.folder_service.find(folder_id) {
            Ok(folder) => folder,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_delete(found) {
            return Err(Error::Forbidden);
        }

        match self.folder_service.delete(folder_id) {
            Ok(folder) => Ok(folder),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }
}
