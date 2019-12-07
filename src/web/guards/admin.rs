use super::auth::Auth;
use entities::models::User;
use rocket::http::Status;
use rocket::request;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;

#[derive(Clone)]
pub struct Admin(User);

impl Admin {
    pub fn user(self) -> User {
        self.0
    }
}

#[derive(Debug)]
pub struct AdminError;

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = AdminError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let auth = match request.guard::<Auth>() {
            Outcome::Success(auth) => auth,
            _ => return Outcome::Failure((Status::Forbidden, AdminError)),
        };

        if auth.clone().user().is_admin() {
            Outcome::Success(Admin(auth.user()))
        } else {
            Outcome::Failure((Status::Forbidden, AdminError))
        }
    }
}
