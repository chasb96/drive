use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use crate::web::state::State;

impl<'a, 'r> FromRequest<'a, 'r> for State<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(Self::from(request.cookies()))
    }
}
