use super::ControllerError as Error;
use entities::models::File;
use entities::diesel::DbFacade;
use diesel::result;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use policies::Restricted;
use schema::*;
use std::fs;
use crate::services::FileService;
use services::error::ServiceError;
use entities::models::User;
use crate::services::StorageService;
use crate::services::file::CreateRequest;
use crate::services::file::UpdateRequest;

pub struct FileController<S: FileService> {
    file_service: S
}

pub struct StoreRequest {
    pub user: User,
    pub name: String,
    pub extension: String,
    pub user_id: i32,
    pub folder_id: i32,
    pub public: bool,
    pub input: fs::File,
}

impl<T: FileService> FileController<T> {
    pub fn new(file_service: T) -> Self {
        Self { file_service }
    }

    pub fn index(&self, user: User, folder_id: i32) -> Result<Vec<File>, Error> {
        if !user.can_index::<File>() {
            log!(
                "info",
                "403 Forbidden. Indexing Files not allowed for user {}",
                user.id()
            );
            return Err(Error::Forbidden);
        }

        match self.file_service.all(folder_id)
        {
            Ok(folders) => Ok(folders),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                Err(Error::InternalServerError)
            }
        }
    }

    pub fn show(&self, user: User, file_id: i32) -> Result<File, Error> {
        let found: File = match self.file_service.find(file_id) {
            Ok(file) => file,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if user.can_view(found.clone()) {
            Ok(found)
        } else {
            log!(
                "info",
                "403 Forbidden. Viewing Files not allowed for user {}",
                user.id()
            );

            Err(Error::Forbidden)
        }
    }

    pub fn create(&self, user: User) -> Result<(), Error> {
        if user.can_create::<File>() {
            Ok(())
        } else {
            log!(
                "info",
                "403 Forbidden. Creating Files not allowed for user {}",
                user.id()
            );

            Err(Error::Forbidden)
        }
    }

    pub fn store(&self, request: StoreRequest) -> Result<File, Error> {
        let file_service = resolve!(FileService);
        let storage_service = resolve!(StorageService);

        if !request.user.can_create::<File>() {
            return Err(Error::Forbidden);
        }

        let file_name = storage_service.store(request.user_id.to_string(), request.input).unwrap();

        let file_create_request = CreateRequest {
            name: request.name,
            extension: request.extension,
            file_name,
            folder_id: request.folder_id,
            public: request.public,
        };

        match file_service.create(file_create_request) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                Err(Error::InternalServerError)
            }
        }
    }

    pub fn edit(&self, user: User, file_id: i32) -> Result<File, Error> {
        let conn = &DbFacade::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if user.can_modify(found.clone()) { Ok(found) }
        else { Err(Error::Forbidden) }
    }

    pub fn update(
        &self,
        user: User,
        file_id: i32,
        name: String,
        extension: String,
        public: bool,
        folder_id: i32,
    ) -> Result<File, Error> {
        let file_service = resolve!(FileService);
        let conn = &DbFacade::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_modify(found.clone()) {
            return Err(Error::Forbidden);
        }

        let file_update_request = UpdateRequest {
            id: file_id,
            name,
            file_name: found.file_name().to_string(),
            extension,
            folder_id,
            public
        };

        match file_service.update(file_update_request) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                Err(Error::InternalServerError)
            }
        }
    }

    pub fn delete(&self, user: User, file_id: i32) -> Result<File, Error> {
        let file_service = resolve!(FileService);
        let storage_service = resolve!(StorageService);

        let conn = &DbFacade::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_delete(found.clone()) {
            log!(
                "info",
                "403 Forbidden. Deletion not allowed for user {}",
                user.id()
            );
            return Err(Error::Forbidden);
        }

        if storage_service.delete(user.id().to_string(), found.file_name().to_string()).is_err()
        {
            return Err(Error::InternalServerError);
        }

        match file_service.delete(file_id) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                Err(Error::InternalServerError)
            }
        }
    }

    pub fn contents(&self, user: User, file_id: i32) -> Result<fs::File, Error> {
        let storage_service = resolve!(StorageService);
        let conn = &DbFacade::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        let owner: i32 = match found.folder() {
            Ok(folder) => folder.user_id(),
            Err(e) => {
                log!("error", "500 Internal Server Erro: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_view(found.clone()) {
            return Err(Error::Forbidden);
        }

        match storage_service.read(owner.to_string(), found.file_name().to_string()) {
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
