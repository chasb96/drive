use bcrypt::hash;
use db::builders::{Builder, UserBuilder};
use db::models::User;
use db::query::Query;
use db::DbFacade;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use env::Env;
use schema::*;
use crate::services::UserService;
use crate::services::FolderService;
use crate::services::FileService;

pub struct Service;

impl UserService for Service {
    fn create(
        name: String,
        email: String,
        role: String,
        password: String,
    ) -> Result<User, Error> {
        let password_hash = hash(&password, Env::bcrypt_cost()).unwrap();

        let user = UserBuilder::new()
            .with_name(name)
            .with_email(email)
            .with_role(role)
            .with_password(password_hash)
            .build()
            .save()?;

        if let Err(e) = <resolve!(FolderService)>::create("/".to_string(), user.id(), None) {
            log!(
                "error",
                "Failed to create root directory for user {}: {}",
                user.id(),
                e
            );
            // return Err(e);
        }

        Ok(user)
    }

    fn update(id: i32, name: String, email: String, role: String) -> Result<User, Error> {
        let mut user = User::all()
            .filter(users::id.eq(id))
            .first::<User>(&DbFacade::connection())?;

        user.set_name(name);
        user.set_email(email);
        user.set_role(role);

        user.update()
    }

    fn delete(id: i32) -> Result<User, Error> {
        let file_service = resolve!(FileService);
        let user = User::all()
            .filter(users::id.eq(id))
            .first::<User>(&DbFacade::connection())?;

        for folder in user.folders()? {
            for file in folder.files()? {
                if let Err(e) = file_service.delete(file.id()) {
                    log!("error", "Failed to delete file {}: {}", file.id(), e);
                    // return Err(e);
                }
            }

            if let Err(e) = <resolve!(FolderService)>::delete(folder.id()) {
                log!("error", "Failed to delete folder {}: {}", folder.id(), e);
                return Err(e);
            }
        }

        user.delete()
    }

    fn update_password(id: i32, password: String) -> Result<User, Error> {
        let mut user = User::all()
            .filter(users::id.eq(id))
            .first::<User>(&DbFacade::connection())?;

        let password_hash = hash(&password, Env::bcrypt_cost()).unwrap();

        user.set_password(password_hash);

        user.update_password()
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_create() {
    //     dotenv::dotenv().expect("Missing .env file");
    //     let conn = DbFacade::connection();
    //
    //     let user = factory!(User);
    //
    //     let actual = Service::create(
    //         user.name().to_string(),
    //         user.email().to_string(),
    //         "guest".to_string(),
    //         user.password().to_string(),
    //     )
    //     .unwrap();
    //
    //     let root = Folder::all()
    //         .filter(folders::user_id.eq(actual.id()))
    //         .filter(folders::parent_id.is_null())
    //         .first::<Folder>(&conn);
    //
    //     assert_eq!(user.name(), actual.name());
    //     assert_eq!(user.email(), actual.email());
    //     assert!(verify(user.password(), actual.password()).unwrap());
    //     assert!(root.is_ok());
    // }
    //
    // #[test]
    // fn test_update() {
    //     dotenv::dotenv().expect("Missing .env file");
    //
    //     let user = factory!(User).save().unwrap();
    //
    //     let expected = factory!(User);
    //     let actual = Service::update(
    //         user.id(),
    //         expected.name().to_string(),
    //         expected.email().to_string(),
    //         "guest".to_string(),
    //     )
    //     .unwrap();
    //
    //     assert_eq!(user.id(), actual.id());
    //     assert_eq!(expected.name(), actual.name());
    //     assert_eq!(expected.email(), actual.email());
    // }
    //
    // #[test]
    // fn test_delete() {
    //     dotenv::dotenv().expect("Missing .env file");
    //     let conn = DbFacade::connection();
    //
    //     let expected = factory!(User).save().unwrap();
    //     let actual = Service::delete(expected.id()).unwrap();
    //
    //     let lookup = User::all()
    //         .filter(users::id.eq(actual.id()))
    //         .first::<User>(&conn);
    //
    //     assert_eq!(expected, actual);
    //     assert_eq!(lookup, Err(Error::NotFound));
    // }
}
