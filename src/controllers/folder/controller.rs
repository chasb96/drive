use crate::controllers::error::ControllerError as Error;
use entities::models::{Folder, User};
use crate::services::folder::CreateRequest as ServiceCreateRequest;
use crate::services::folder::UpdateRequest as ServiceUpdateRequest;
use crate::services::error::ServiceError;
use crate::controllers::folder::FolderController;
use crate::services::FolderService;
use crate::controllers::folder::StoreRequest;
use crate::controllers::folder::UpdateRequest;
use policies::folder::FolderAuthorizer;

pub struct Controller<T: FolderService, S: FolderAuthorizer> {
    folder_service: T,
    folder_authorizer: S
}

impl<T: FolderService, S: FolderAuthorizer> Controller<T, S> {
    pub fn new(folder_service: T, folder_authorizer: S) -> Self {
        Self {
            folder_service,
            folder_authorizer
        }
    }
}

impl<T: FolderService, S: FolderAuthorizer> FolderController for Controller<T, S> {
    fn index(&self, user: User, parent_id: Option<i32>) -> Result<Vec<Folder>, Error> {
        // Verify that the user has permission to view folders
        //
        // This should never atm be false, but I'd like the
        //  ability to restrict this in the future
        if !self.folder_authorizer.can_index(&user) {
            return Err(Error::Forbidden);
        }

        // Get all of the folders belonging to the
        //  parent provided
        //
        // In a NotFound we return that,
        //  but we want to hide 500 server errors from
        //  the outside world
        //
        // There's no convenient query we can run giving
        //  an optional parent - that'd be two different
        //  queries alltogether, so we fetch all of them
        //  and filter them out ourself
        let mut folders = match self.folder_service.all(user.id()) {
            Ok(folders) => folders,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        // Filter out the folders where the parent_id matches
        //  the one provided
        match parent_id {
            Some(parent_id) => folders.retain(| folder | folder.id() == parent_id),
            None => folders.retain(| folder | folder.parent_id().is_none()),
        };

        Ok(folders)
    }

    fn show(&self, user: User, folder_id: i32) -> Result<Folder, Error> {
        // Get the folder from the datastore given the
        //  folder_id provided.
        //
        // If there's no such folder, return NotFound
        //  otherwise, we want to hide what happened
        //
        // We need to actually have the folder first to
        //  validate the user has access
        let found: Folder = match self.folder_service.find(folder_id) {
            Ok(folder) => folder,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        // Validate that the user has access to the
        //  folder, if not, forbid it
        if self.folder_authorizer.can_view(&user, &found) {
            Ok(found)
        } else {
            Err(Error::Forbidden)
        }
    }

    fn create(&self, user: User) -> Result<(), Error> {
        // This method is accessed as a `Create` page,
        //  but there's no actual data for Creates, only
        //  for actually storing the data. So we only
        //  validate authorization here
        if self.folder_authorizer.can_create(&user) {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    fn store(&self, user: User, request: StoreRequest) -> Result<Folder, Error> {
        // Validate the user has authorization beforehand
        //  if they don't then we prevent a lot of work
        //  ahead of time
        //
        // Atm, this will always be true, but I'd like
        //  the ability to restrict this if I wanted
        if !self.folder_authorizer.can_create(&user) {
            return Err(Error::Forbidden);
        }

        let service_create_request = ServiceCreateRequest {
            name: request.name,
            user_id: request.user_id,
            parent_id: request.parent_id,
        };

        // Attempt to create the folder. There's nothing
        //  we need to do with the folder so we can immediately
        //  return it back.
        //
        // We want to hide errors beyond here, so log it and
        //  don't yield any info on the error
        match self.folder_service.create(service_create_request) {
            Ok(folder) => Ok(folder),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            },
        }
    }

    fn edit(&self, user: User, folder_id: i32) -> Result<Folder, Error> {
        // Find the folder by id
        //
        // If the folder can't be found, then throw a NotFound
        // If there's an error, hide the error from here
        let found: Folder = match self.folder_service.find(folder_id) {
            Ok(folder) => folder,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        // Verify that the user has permission to modify
        //  this folder. If not, then throw back a Forbidden
        if self.folder_authorizer.can_modify(&user, &found) {
            Ok(found)
        } else {
            Err(Error::Forbidden)
        }
    }

    fn update(&self, user: User, request: UpdateRequest) -> Result<Folder, Error> {
        // Find the folder given the folder_id specified
        //  within the request.
        //
        // If there's no folder, throw a NotFound
        // Upon error, throw back an internal server error
        let found: Folder = match self.folder_service.find(request.folder_id) {
            Ok(folder) => folder,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                return Err(Error::InternalServerError);
            }
        };

        // If the user cannot modify this Folder
        //  stop here
        if !self.folder_authorizer.can_modify(&user, &found) {
            return Err(Error::Forbidden);
        }

        let service_update_request = ServiceUpdateRequest {
            id: request.folder_id,
            name: request.name,
            user_id: request.user_id,
            parent_id: request.parent_id,
        };

        // Update the folder
        // Again, on error, throw back an InternalServerError
        match self.folder_service.update(service_update_request) {
            Ok(folder) => Ok(folder),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }

    fn delete(&self, user: User, folder_id: i32) -> Result<Folder, Error> {
        // Find the folder given the folder_id specified
        //  within the request.
        //
        // If there's no folder, throw a NotFound
        // Upon error, throw back an internal server error
        let found: Folder = match self.folder_service.find(folder_id) {
            Ok(folder) => folder,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                return Err(Error::InternalServerError);
            }
        };

        // If the user cannot delete this folder
        //  throw back a forbidden
        if !self.folder_authorizer.can_delete(&user, &found) {
            return Err(Error::Forbidden);
        }

        // Delete the folder
        // On an error, throw back the error
        match self.folder_service.delete(folder_id) {
            Ok(folder) => Ok(folder),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }
}
