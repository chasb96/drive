use crate::policies::user::UserAuthorizer;
use crate::controllers::user::UpdateRequest;
use crate::controllers::user::StoreRequest;
use crate::controllers::user::UserController;
use crate::services::user::CreateRequest as ServiceCreateRequest;
use crate::services::user::UpdateRequest as ServiceUpdateRequest;
use crate::controllers::error::ControllerError as Error;
use crate::entities::models::User;
use crate::services::UserService;
use crate::services::error::ServiceError;

pub struct Controller<T: UserService, S: UserAuthorizer> {
    user_service: T,
    user_authorizer: S
}

impl<T: UserService, S: UserAuthorizer> Controller<T, S> {
    pub fn new(user_service: T, user_authorizer: S) -> Self {
        Self {
            user_service,
            user_authorizer,
        }
    }
}

impl<T: UserService, S: UserAuthorizer> UserController for Controller<T, S> {
    fn index(&self, user: User) -> Result<Vec<User>, Error> {
        if !self.user_authorizer.can_index(&user) {
            return Err(Error::Forbidden);
        }

        match self.user_service.all() {
            Ok(users) => Ok(users),
            Err(_) => Err(Error::InternalServerError),
        }
    }

    fn show(&self, user: User, user_id: i32) -> Result<User, Error> {
        // Attempt to find the user by their user_id,
        //  if it's not found, throw back the error
        let found: User = match self.user_service.find_by_user_id(user_id) {
            Ok(user) => user,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if self.user_authorizer.can_view(&user, &found) {
            Ok(found)
        } else {
            Err(Error::Forbidden)
        }
    }

    fn create(&self, user: User) -> Result<(), Error> {
        if self.user_authorizer.can_create(&user) {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    fn store(&self, user: User, request: StoreRequest) -> Result<User, Error> {
        // We already know the context of the User,
        //  first verify they can create Users
        if !self.user_authorizer.can_create(&user) {
            return Err(Error::Forbidden);
        }

        let create_request = ServiceCreateRequest {
            name: request.name,
            email: request.email,
            role: request.role,
            password: request.password
        };

        match self.user_service.create(create_request) {
            Ok(user) => Ok(user),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }

    fn edit(&self, user: User, user_id: i32) -> Result<User, Error> {
        // Attempt to find the user by their user_id,
        //  if not found, throw back the error
        let found: User = match self.user_service.find_by_user_id(user_id) {
            Ok(user) => user,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if self.user_authorizer.can_modify(&user, &found) {
            Ok(found)
        } else {
            Err(Error::Forbidden)
        }
    }

    fn update(&self, user: User, request: UpdateRequest) -> Result<User, Error> {
        // Attempt to find the user by their user_id,
        //  if there is an error, like NotFound,
        //  throw back the error
        let found: User = match self.user_service.find_by_user_id(request.user_id) {
            Ok(user) => user,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        // Verify the user can modify this user
        if !self.user_authorizer.can_modify(&user, &found) {
            return Err(Error::Forbidden);
        }

        let request = ServiceUpdateRequest {
            id: request.user_id,
            name: request.name,
            email: request.email,
            role: request.role
        };

        match self.user_service.update(request) {
            Ok(user) => Ok(user),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }

    fn delete(&self, user: User, user_id: i32) -> Result<User, Error> {
        // Attempt to find the user by their user_id,
        //  if there's an error, throw it back
        let found: User = match self.user_service.find_by_user_id(user_id) {
            Ok(user) => user,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        // Verify that the user can delete this user
        if !self.user_authorizer.can_delete(&user, &found) {
            return Err(Error::Forbidden);
        }

        match self.user_service.delete(user_id) {
            Ok(user) => Ok(user),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }

    fn update_password(&self, user: User, user_id: i32, password: String) -> Result<User, Error> {
        // Attempt to find the user by their user_id,
        //  if they cannot be found, throw back the error
        let found: User = match self.user_service.find_by_user_id(user_id) {
            Ok(user) => user,
            Err(ServiceError::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        // Verify that the user has permissions to update users
        if !self.user_authorizer.can_modify(&user, &found) {
            return Err(Error::Forbidden);
        }

        match self.user_service.update_password(user_id, password) {
            Ok(user) => Ok(user),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);

                Err(Error::InternalServerError)
            }
        }
    }
}
