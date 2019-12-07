use super::context::Context;
use super::error::Error;
use super::success::Success;
use rocket::http::{Cookie, Cookies};
use serde::Serialize;

pub struct State<'a>(Cookies<'a>);

impl<'a> State<'a> {
    pub fn cookies(self) -> Cookies<'a> {
        self.0
    }

    pub fn errors(&mut self) -> Vec<Error> {
        match &self.0.get_private("errors") {
            Some(cookie) => {
                let errors: Vec<Error> =
                    cookie.value().split("||").map(Error::from).collect();

                self.0.remove_private(Cookie::named("errors"));

                errors
            }
            None => Vec::new(),
        }
    }

    pub fn push_error(&mut self, error: Error) {
        let mut errors = self.errors();

        errors.push(error);

        let cookie_value = errors
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("||");

        self.0.add_private(Cookie::new("errors", cookie_value));
    }

    pub fn successes(&mut self) -> Vec<Success> {
        match &self.0.get_private("successes") {
            Some(cookie) => {
                let successes: Vec<Success> = cookie
                    .value()
                    .split("||")
                    .map(Success::from)
                    .collect();

                self.0.remove_private(Cookie::named("successes"));

                successes
            }
            None => Vec::new(),
        }
    }

    pub fn push_success(&mut self, success: Success) {
        let mut successes = self.successes();

        successes.push(success);

        let cookie_value = successes
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("||");

        self.0.add_private(Cookie::new("successes", cookie_value));
    }

    pub fn into_context<T: Serialize>(mut self, data: T) -> Context<T> {
        Context::new(self.errors(), self.successes(), data)
    }
}

impl<'a> From<Cookies<'a>> for State<'a> {
    fn from(from: Cookies<'a>) -> Self {
        Self(from)
    }
}
