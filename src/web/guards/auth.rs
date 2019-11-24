use super::token_convert::TokenError;
use auth::authenticate::Authenticate;
use auth::bearer::Token;
use auth::Auth as InternalAuth;
use entities::models::User;
use rocket::http::{Cookie, Status};
use rocket::request;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use std::convert::TryFrom;

#[derive(Clone)]
pub struct Auth(User);

impl Auth {
    pub fn user(self) -> User {
        self.0
    }
}

#[derive(Debug)]
pub struct AuthError;

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let headers = request.headers();
        let mut refresh_token = false;

        let token: Token = match Token::try_from(request.cookies())
            .and_then(|token| {
                refresh_token = true;
                Ok(token)
            })
            .or_else(|_| Token::try_from(headers))
        {
            Ok(token) => token,
            Err(TokenError::NotPresent) => {
                return Outcome::Failure((Status::Unauthorized, AuthError))
            }
            Err(TokenError::Invalid) => return Outcome::Failure((Status::BadRequest, AuthError)),
        };

        let user: User = match InternalAuth::Bearer(token).verify() {
            Ok(user) => user,
            Err(_) => return Outcome::Failure((Status::Unauthorized, AuthError)),
        };

        if refresh_token {
            let mut cookies = request.cookies();

            let token = match Token::try_from(user.clone()) {
                Ok(token) => token,
                Err(e) => {
                    log!("error", "500 Internal Server Error: {}", e);
                    return Outcome::Failure((Status::InternalServerError, AuthError));
                }
            };

            cookies.remove_private(Cookie::named("token"));
            cookies.add_private(Cookie::new("token", token.to_string()));
        }

        Outcome::Success(Auth(user))
    }
}
