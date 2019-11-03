use super::ControllerError as Error;
use db::models::{File, User};
use db::DbFacade;
use diesel::result;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use policies::Restricted;
use schema::*;
use std::fs;
use crate::services::FileService;

pub struct FileController<T: FileService> {
    file_service: T
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
        let conn = &DbFacade::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        match user.can_view(found.clone()) {
            true => Ok(found),
            false => {
                log!(
                    "info",
                    "403 Forbidden. Viewing Files not allowed for user {}",
                    user.id()
                );
                Err(Error::Forbidden)
            }
        }
    }

    pub fn create(&self, user: User) -> Result<(), Error> {
        match user.can_create::<File>() {
            true => Ok(()),
            false => {
                log!(
                    "info",
                    "403 Forbidden. Creating Files not allowed for user {}",
                    user.id()
                );
                Err(Error::Forbidden)
            }
        }
    }

    pub fn store(
        &self,
        user: User,
        name: String,
        extension: String,
        user_id: i32,
        folder_id: i32,
        public: bool,
        input: fs::File,
    ) -> Result<File, Error> {
        let file_service = resolve!(FileService);
        if !user.can_create::<File>() {
            return Err(Error::Forbidden);
        }

        let file_name = <resolve!(StorageService)>::store(user_id.to_string(), input).unwrap();

        match file_service.create(name, extension, file_name, folder_id, public) {
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

        match user.can_modify(found.clone()) {
            true => Ok(found),
            false => Err(Error::Forbidden),
        }
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

        match file_service.update(
            file_id,
            name,
            found.file_name().to_string(),
            extension,
            folder_id,
            public,
        ) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        }
    }

    pub fn delete(&self, user: User, file_id: i32) -> Result<File, Error> {
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

        if !user.can_delete(found.clone()) {
            log!(
                "info",
                "403 Forbidden. Deletion not allowed for user {}",
                user.id()
            );
            return Err(Error::Forbidden);
        }

        if let Err(_) = <resolve!(StorageService)>::delete(user.id().to_string(), found.file_name().to_string())
        {
            return Err(Error::InternalServerError);
        }

        match file_service.delete(file_id) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        }
    }

    pub fn contents(&self, user: User, file_id: i32) -> Result<fs::File, Error> {
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

        match <resolve!(StorageService)>::read(owner.to_string(), found.file_name().to_string()) {
            Ok(contents) => Ok(contents),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use db::builders::*;
    use std::error::Error;
    use super::FileController;

    #[test]
    fn test_index() -> Result<(), Box<dyn Error>> {
        let user = factory!(User);
        let folder = factory!(Folder, user.id(), None);
        let expected = vec![
            factory!(File, folder.id()),
            factory!(File, folder.id()),
            factory!(File, folder.id()),
            factory!(File, folder.id()),
            factory!(File, folder.id()),
            factory!(File, folder.id()),
        ];

        let mut file_service = resolve!(FileService);
        file_service
            .expect_all(| folder_id | folder_id.partial_eq(folder.id()))
            .returns(Ok(expected.clone()));

        let file_controller = FileController::new(file_service);

        let actual = file_controller.index(user, folder.id())?;

        assert_eq!(expected, actual);

        Ok(())
    }
}
