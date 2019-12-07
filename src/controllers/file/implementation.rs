use controllers::file::UpdateRequest;
use controllers::file::StoreRequest;
use controllers::file::FileController;
use crate::controllers::error::ControllerError as Error;
use entities::models::File;
use policies::Restricted;
use std::fs;
use crate::services::FileService;
use services::error::ServiceError;
use entities::models::User;
use crate::services::StorageService;
use crate::services::file::CreateRequest as ServiceCreateRequest;
use crate::services::file::UpdateRequest as ServiceUpdateRequest;

pub struct Controller<T: FileService, S: StorageService> {
    file_service: T,
    storage_service: S
}

impl<T: FileService, S: StorageService> Controller<T, S> {
    pub fn new(file_service: T, storage_service: S) -> Self {
        Self {
            file_service,
            storage_service,
        }
    }
}

impl<T: FileService, S: StorageService> FileController for Controller<T, S> {
    fn index(&self, _: User, folder_id: i32) -> Result<Vec<File>, Error> {
        match self.file_service.all(folder_id)
        {
            Ok(folders) => Ok(folders),
            Err(e) => {
                // If something went wrong, it needs to be logged
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }

    fn show(&self, user: User, file_id: i32) -> Result<File, Error> {
        // Attempt to get the File requested by its Id
        // If the File cannot be found, then return NotFound
        // If there's an error, log it and throw back the error
        let found: File = match self.file_service.find(file_id) {
            Ok(file) => file,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                return Err(Error::InternalServerError);
            }
        };

        // Files are per-user and are restricted by permissions
        //  If the user does not have permission to view the file,
        //  then throw a NotFound
        //
        // We through a NotFound instead of Forbidden
        //  as we don't want to yield that this file
        //  actually exists if they cannot access it
        if user.can_view(found.clone()) {
            Ok(found)
        } else {
            Err(Error::NotFound)
        }
    }

    fn create(&self, user: User) -> Result<(), Error> {
        if user.can_create::<File>() {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    fn store(&self, user: User, request: StoreRequest) -> Result<File, Error> {
        // Verify that the user can create files first.
        // If they cannot, return back a Forbidden
        if !user.can_create::<File>() {
            return Err(Error::Forbidden);
        }

        // Attempt to store the file,
        //  if it fails, log it and throw back the error
        let file_name = match self.storage_service.store(request.user_id.to_string(), request.input) {
            Ok(file_name) => file_name,
            Err(e) => {
                log!("error", "Internal Server Error: {}", e);

                return Err(Error::from(e));
            }
        };

        let file_create_request = ServiceCreateRequest {
            name: request.name,
            extension: request.extension,
            file_name,
            folder_id: request.folder_id,
            public: request.public,
        };

        // Attempt to store the file in the DataStore,
        //  if it fails, log it and throw it back
        match self.file_service.create(file_create_request) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }

    fn edit(&self, user: User, file_id: i32) -> Result<File, Error> {
        // Attempt to get the file by its Id
        // If it's not found, return back a NotFound,
        // If there's an error, log it, and throw it back
        let found: File = match self.file_service.find(file_id) {
            Ok(file) => file,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                return Err(Error::InternalServerError);
            }
        };

        // If the user it not allowed to modify
        //  this file, throw a NotFound instead
        //
        // We through a NotFound instead of Forbidden
        //  as we don't want to yield that this file
        //  actually exists if they cannot access it
        if user.can_modify(found.clone()) {
            Ok(found)
        } else {
            Err(Error::NotFound)
        }
    }

    fn update(&self, user: User, request: UpdateRequest) -> Result<File, Error> {
        // Attempt to get the file by its Id
        // If it's not found, return back a NotFound,
        // If there's an error, log it, and throw it back
        let found: File = match self.file_service.find(request.file_id) {
            Ok(file) => file,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        // If the user cannot modify this,
        //  we simply don't want to modfiy it
        //
        // Instead of Forbidden, if the user can't
        //  modify this, we don't want to reveal that the
        //  option exists, so we throw a NotFound instead
        if !user.can_modify(found.clone()) {
            return Err(Error::NotFound);
        }

        let file_update_request = ServiceUpdateRequest {
            id: request.file_id,
            name: request.name,
            file_name: found.file_name().to_string(),
            extension: request.extension,
            folder_id: request.folder_id,
            public: request.public
        };

        // Attempt to update the file,
        // If something goes wrong, log it and throw it back
        match self.file_service.update(file_update_request) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                Err(Error::InternalServerError)
            }
        }
    }

    fn delete(&self, user: User, file_id: i32) -> Result<File, Error> {
        // Attempt to find the file by its Id
        // If it's not found, return a not found,
        //  in case of error, throw back an InternalServerError
        let found: File = match self.file_service.find(file_id) {
            Ok(file) => file,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        // If the user is not allowed to access this file,
        //  we don't to show that the behavior exists
        if !user.can_delete(found.clone()) {
            return Err(Error::NotFound);
        }

        // Attempt to delete the file from storage
        //  on failure, give back an internal server error
        if self.storage_service.delete(user.id().to_string(), found.file_name().to_string()).is_err()
        {
            return Err(Error::InternalServerError);
        }

        // Delete the file from the DataStore
        match self.file_service.delete(file_id) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                Err(Error::InternalServerError)
            }
        }
    }

    fn contents(&self, user: User, file_id: i32) -> Result<fs::File, Error> {
        // Attempt to find the file by its Id
        // On NotFound, give a NotFound,
        // On error, mask the error as an InternalServerError
        let found: File = match self.file_service.find(file_id) {
            Ok(file) => file,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        // If the user cannot view this file,
        //  mask it behind a NotFound
        if !user.can_view(found.clone()) {
            return Err(Error::NotFound);
        }

        // This is less obvious than it looks:
        // There's no guarantee here that the file technically
        //  corresponds to this User, other than the above assertions
        // But, if the above fails, it'll still only try to yield
        //  something of their ownership.
        // This is an impossible case, so we just return a 500 here
        //  on error
        match self.storage_service.read(user.id().to_string(), found.file_name().to_string()) {
            Ok(contents) => Ok(contents),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                Err(Error::InternalServerError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use entities::builders::*;
    // use std::error::Error;
    // use super::FileController;
    //
    // #[test]
    // fn test_index() -> Result<(), Box<dyn Error>> {
    //     let user = factory!(User);
    //     let folder = factory!(Folder, user.id(), None);
    //     let expected = vec![
    //         factory!(File, folder.id()),
    //         factory!(File, folder.id()),
    //         factory!(File, folder.id()),
    //         factory!(File, folder.id()),
    //         factory!(File, folder.id()),
    //         factory!(File, folder.id()),
    //     ];
    //
    //     let mut file_service = resolve!(FileService);
    //     file_service
    //         .expect_all()
    //         .return_const(Ok(expected.clone()));
    //
    //     let file_controller = FileController::new(file_service);
    //
    //     let actual = file_controller.index(user, folder.id())?;
    //
    //     assert_eq!(expected, actual);
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_show() -> Result<(), Box<dyn Error>> {
    //     let user = factory!(User);
    //     let folder = factory!(Folder, user.id(), None);
    //     let expected = factory!(File, folder.id());
    //
    //     let mut file_service = resolve!(FileService);
    //     file_service
    //         .expect_find()
    //         .return_const(Ok(expected.clone()));
    //
    //     let file_controller = FileController::new(file_service);
    //
    //     let actual = file_controller.show(user, expected.id())?;
    //
    //     assert_eq!(expected, actual);
    //
    //     Ok(())
    // }
}
