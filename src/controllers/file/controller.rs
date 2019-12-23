use crate::policies::file::FileAuthorizer;
use crate::controllers::file::UpdateRequest;
use crate::controllers::file::StoreRequest;
use crate::controllers::file::FileController;
use crate::controllers::error::ControllerError as Error;
use crate::entities::models::File;
use std::fs;
use crate::services::FileService;
use crate::services::error::ServiceError;
use crate::entities::models::User;
use crate::services::StorageService;
use crate::services::file::CreateRequest as ServiceCreateRequest;
use crate::services::file::UpdateRequest as ServiceUpdateRequest;

pub struct Controller<T: FileService, S: StorageService, R: FileAuthorizer> {
    file_service: T,
    storage_service: S,
    file_authorizer: R,
}

impl<T: FileService, S: StorageService, R: FileAuthorizer> Controller<T, S, R> {
    pub fn new(file_service: T, storage_service: S, file_authorizer: R) -> Self {
        Self {
            file_service,
            storage_service,
            file_authorizer
        }
    }
}

impl<T: FileService, S: StorageService, R: FileAuthorizer> FileController for Controller<T, S, R> {
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
        if self.file_authorizer.can_view(&user, &found) {
            Ok(found)
        } else {
            Err(Error::NotFound)
        }
    }

    fn create(&self, user: User) -> Result<(), Error> {
        if self.file_authorizer.can_create(&user) {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    fn store(&self, user: User, request: StoreRequest) -> Result<File, Error> {
        // Verify that the user can create files first.
        // If they cannot, return back a Forbidden
        if !self.file_authorizer.can_create(&user) {
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
        if self.file_authorizer.can_modify(&user, &found) {
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
        if !self.file_authorizer.can_modify(&user, &found) {
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
        //  if something goes wrong, log it and throw it back
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
        //
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
        if !self.file_authorizer.can_delete(&user, &found) {
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
        if !self.file_authorizer.can_view(&user, &found) {
            return Err(Error::NotFound);
        }

        // This is less obvious than it looks:
        //
        // There's no guarantee here that the file technically
        //  corresponds to this User, other than the above assertions
        //
        // But, if the above fails, it'll still only try to yield
        //  something of their ownership.
        //
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
